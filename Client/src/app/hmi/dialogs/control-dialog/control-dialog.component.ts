import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { ButtonFunction, CommandAction } from '../../../shared/hmi.constants'
import { Symbol } from '../../../shared/hmi.constants'
import { DiagramData } from '../../../shared/models/userobject.model';
import { getCommands, getCommandsByType } from '../../../shared/models/commands.model';

@Component({
  selector: 'app-control-dialog',
  templateUrl: './control-dialog.component.html',
  styleUrls: ['./control-dialog.component.scss']
})
export class ControlDialogComponent implements OnInit {  
  setpointValue: number;
  controlValue: any;
  isSetPoint: boolean;
  isFixedCommand: boolean;
  name: string;  
  diagramId: string;
  mRID: string; 
  type: string;  
  diagramData: DiagramData;
  isControllable: boolean = true;
  hasDataMapped: boolean = false;
  lastUpdate: string;
  hasLastUpdate: boolean = false;
  commands: any[] = [];

  constructor(
    public dialogRef: MatDialogRef<ControlDialogComponent>,    
    @Inject(MAT_DIALOG_DATA) public data: any
  ) { }

  ngOnInit() {  
    let commandTypes = getCommands();
    for (let entry of commandTypes) {
      let a = getCommandsByType(entry);

      for (let cmd of a) {
        if (cmd.attributes && cmd.attributes.name) {
          this.commands.push(cmd.attributes.name);
        }
      }      
    }

    this.diagramId = this.data.diagramId;

    this.diagramData = this.data.diagramData;
    this.setpointValue = this.diagramData.tag;
    this.name = this.diagramData.name,
    this.mRID = this.diagramData.mRID; 
    this.lastUpdate = this.diagramData.lastUpdate;
    if (this.lastUpdate) {
      this.hasLastUpdate = true;
    }   
    
    this.type = this.diagramData.type;        
    this.isSetPoint = this.type === Symbol.setPointButton;
    
    if (this.type == Symbol.button) {
      if (this.diagramData.func === ButtonFunction.command && this.diagramData.verb)
      {
        this.hasDataMapped = true; 
        this.controlValue = this.diagramData.verb;       
      }
      else {
        this.isControllable = false;
        this.hasDataMapped = false;
      }
    }
    else if (!this.diagramData.controlData || this.diagramData.controlData.length == 0) {
      this.isControllable = false;
      this.hasDataMapped = false;
    }
    else {
      this.hasDataMapped = true;
      if (!this.isSetPoint) {
        var dataType = this.diagramData.controlData[0].type;
        if (dataType === 'binary') {
          this.controlValue = this.diagramData.controlData[0].measurement;
          if (!this.controlValue || this.controlValue === '') {
            this.isControllable = false;
          }
        }
        else if (dataType === 'analog') {
          this.isSetPoint = true;
          this.setpointValue = this.diagramData.controlData[0].measurement;
        }    
        else if (this.commands.includes(this.diagramData.controlData[0].path)) {            
            this.isControllable = true;
            this.hasDataMapped = true;
        }
        else {
          console.error('Data type for control point is not supported: ' + dataType);
          this.isControllable = false;
        }
      }
    }     
  }
  
  onClose(): void {    
    this.dialogRef.close();
  }  

  isNumeric(num: Number) {
    if (typeof num != "number") return false // we only process strings!  
    return !isNaN(num);
  }

  onAction() : void {

    if (this.isSetPoint) {
      if (!this.setpointValue) {
        alert('Please specify setpoint value');
        return;
      }
      else if (!this.isNumeric(this.setpointValue)) {
        alert('Not a valid value');
        return;
      }

      this.dialogRef.close({
        proceed: true,
        action: CommandAction.SETVALUE,
        value: this.setpointValue  
      });
    }
    else if (this.type == Symbol.button)
    {
      this.dialogRef.close({
        proceed: true,
        action: CommandAction.VERB,
        value: this.controlValue  
      });
    }
    else {  // status indicator
      this.dialogRef.close({
        proceed: true,
        action: CommandAction.PRECONFIGURED,
        value: this.controlValue  
      });
    }       
  }
}
