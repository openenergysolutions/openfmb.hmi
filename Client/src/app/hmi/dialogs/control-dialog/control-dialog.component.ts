// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { ButtonFunction, CommandAction } from '../../../shared/hmi.constants'
import { Symbol } from '../../../shared/hmi.constants'
import { DiagramData } from '../../../shared/models/userobject.model';
import { getCommands, getCommandsByType } from '../../../shared/models/commands.model';
import { Authorization } from '../../../shared/models/user.model';
import { JwtAuthService } from "../../../shared/services/auth/jwt-auth.service";

@Component({
  selector: 'app-control-dialog',
  templateUrl: './control-dialog.component.html',
  styleUrls: ['./control-dialog.component.scss']
})
export class ControlDialogComponent implements OnInit {  
  setpointValue: number;
  controlValue: any;
  ggioIndex: any; // used for resource ggio
  isSetPoint: boolean = false;
  isSetBoolean: boolean = false;
  onOffs: boolean[] = [false, true];
  onOffCommand: boolean = false;
  isFixedCommand: boolean;
  name: string;  
  diagramId: string;
  mRID: string;
  description: string;
  showDescription: boolean = false; 
  type: string;  
  diagramData: DiagramData;
  isControllable: boolean = true;
  hasDataMapped: boolean = false;
  lastUpdate: string;
  hasLastUpdate: boolean = false;
  commands: any[] = [];

  constructor(
    public dialogRef: MatDialogRef<ControlDialogComponent>, 
    private jwtService: JwtAuthService,   
    @Inject(MAT_DIALOG_DATA) public data: any
  ) { }

  ngOnInit() {  
    const canControl = Authorization.canControl(this.jwtService.getUserRole());
    let commandTypes = getCommands();
    for (let entry of commandTypes) {
      let a = getCommandsByType(entry);

      for (let cmd of a) {
        if (cmd.attributes && cmd.attributes.name) {          
          this.commands.push(cmd);
        }
      }      
    }

    this.diagramId = this.data.diagramId;

    this.diagramData = this.data.diagramData;
    this.setpointValue = this.diagramData.tag;
    this.name = this.diagramData.name,
    this.mRID = this.diagramData.mRID; 
    this.description = this.diagramData.description;
    this.showDescription = this.description && this.description !== "";
    this.lastUpdate = this.diagramData.lastUpdate;
    if (this.lastUpdate) {
      this.hasLastUpdate = true;
    }   
    
    this.type = this.diagramData.type;        
    //this.isSetPoint = this.type === Symbol.setPointButton;
    console.log(JSON.stringify(this.diagramData));
    if (this.type == Symbol.button) {
      if (this.diagramData.func === ButtonFunction.command)
      {
        this.hasDataMapped = this.diagramData.verb && this.diagramData.verb !== ""; 
        this.controlValue = this.diagramData.verb; 
        this.isSetPoint = false;      
      }
      else if (this.diagramData.func === ButtonFunction.setPoint)
      {
        this.hasDataMapped = this.diagramData.verb && this.diagramData.verb !== "";
        this.isControllable = canControl; 
        this.isSetPoint = true;        
      }
      else {
        // this is link button, not controllable
        this.isControllable = false;
        this.hasDataMapped = false;
        this.isSetPoint = false; 
      }
    }
    else if (!this.diagramData.controlData || this.diagramData.controlData.length == 0) {
      // No data mapping (mapped from "Data Connect" screen, Control tab)
      this.isControllable = false;
      this.hasDataMapped = false;
    }
    else {
      // Has control data mapping (mapped from "Data Connect" screen, Control tab)
      this.hasDataMapped = true;      
      var dataType = this.diagramData.controlData[0].type;
      
      if (dataType === 'binary') {
        this.controlValue = this.diagramData.controlData[0].measurement;
        if (this.controlValue === undefined || this.controlValue === '') {
          this.isControllable = false;
        }
      }
      else if (dataType === 'analog' || dataType === 'FLOAT64' || dataType === 'FLOAT32') {
        this.isSetPoint = true;
        this.setpointValue = this.diagramData.controlData[0].measurement;
      }          
      else {

        var cmd = this.findCommand(this.diagramData.controlData[0].path);
        if (cmd != null) {
          this.isControllable = canControl;
          this.hasDataMapped = true; 
          this.isSetPoint = cmd.attributes.type == "set-point";  
          this.isSetBoolean = cmd.attributes.type == "set-boolean";          
          let index = parseInt( this.diagramData.controlData[0].measurement);       
          if (isNaN(index)) {
            index = 0;
          }
          this.ggioIndex = index;
        }
        else {
          console.error('Data type for control point is not supported: ' + dataType);
          this.isControllable = false;
        }
      }
    }    
  }

  findCommand(name: String) : any {
    for(var i = 0; i < this.commands.length; ++i) {
      if (this.commands[i].attributes.name == name) {
        return this.commands[i];
      }
    }
    return null;
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
      if (!this.isNumeric(this.setpointValue)) {
        alert('Please specify valid numeric setpoint value');
        return;
      }

      this.dialogRef.close({
        proceed: true,
        //action: CommandAction.SETVALUE,
        action: this.diagramData.verb ? this.diagramData.verb : CommandAction.SETVALUE,
        value: this.setpointValue,
        index: this.ggioIndex,  
      });
    }    
    else if (this.isSetBoolean) {        
      this.dialogRef.close({
        proceed: true,        
        action: this.diagramData.verb ? this.diagramData.verb : CommandAction.SETVALUE,
        value: this.onOffCommand ? 1.0 : 0.0,
        index: this.ggioIndex,  
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

  onMessageInspector(): void {
    window.open('/inspector?mrid=' + this.mRID, '_blank', 'toolbar=0,width=850,height=700');
    this.dialogRef.close({
      proceed: false      
    });
  }
}
