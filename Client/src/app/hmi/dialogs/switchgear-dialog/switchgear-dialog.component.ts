import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { DiagramData } from '../../../shared/models/userobject.model'

@Component({
  selector: 'app-switchgear-dialog',
  templateUrl: './switchgear-dialog.component.html',
  styleUrls: ['./switchgear-dialog.component.scss']
})
export class SwitchgearDialogComponent implements OnInit {
  filterData: any;
  status: string;
  name: string;
  actionColor: string;
  actionText: string;
  actionEnabled: boolean = true;
  diagramId: string;
  mRID: string;    

  constructor(
    public dialogRef: MatDialogRef<SwitchgearDialogComponent>,    
    @Inject(MAT_DIALOG_DATA) public data: any
  ) { }

  ngOnInit() {    
    this.filterData = this.data;
    this.status = this.data.status;
    this.name = this.data.name,
    this.mRID = this.data.mRID;    
    this.diagramId = this.data.diagramId;    
    
    this.dialogRef.updatePosition({
      top: `${this.filterData.top}px`,
      left: `${this.filterData.left}px`
    });  
    
    if (this.status && this.status.toLowerCase() === "open") {
      this.actionText = "CLOSE";
      this.actionColor = "red";
    }
    else if (this.status && this.status.toLowerCase() === "closed"){
      this.actionText = "OPEN";
      this.actionColor = "green";
    }
    else {
      this.actionText = "INVALID";
      this.actionColor = "gray";
      this.actionEnabled = false;
    }
  }
  
  onClose(): void {    
    this.dialogRef.close();
  }  

  onAction() : void {
    this.dialogRef.close({
      proceed: true,
      action: this.actionText
    });
  }
}
