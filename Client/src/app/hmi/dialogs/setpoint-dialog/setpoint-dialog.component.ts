import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { CommandAction } from '../../../shared/hmi.constants'

@Component({
  selector: 'app-setpoint-dialog',
  templateUrl: './setpoint-dialog.component.html',
  styleUrls: ['./setpoint-dialog.component.scss']
})
export class SetPointDialogComponent implements OnInit {
  filterData: any;
  setpointValue: number;
  name: string;  
  diagramId: string;
  mRID: string;    

  constructor(
    public dialogRef: MatDialogRef<SetPointDialogComponent>,    
    @Inject(MAT_DIALOG_DATA) public data: any
  ) { }

  ngOnInit() {    
    this.filterData = this.data;
    this.setpointValue = this.data.value;
    this.name = this.data.name,
    this.mRID = this.data.mRID;    
    this.diagramId = this.data.diagramId;    
    
    this.dialogRef.updatePosition({
      top: `${this.filterData.top}px`,
      left: `${this.filterData.left}px`
    });      
    
  }
  
  onClose(): void {    
    this.dialogRef.close();
  }  

  isNumeric(num: Number) {
    if (typeof num != "number") return false // we only process strings!  
    return !isNaN(num);
  }

  onAction() : void {

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
}
