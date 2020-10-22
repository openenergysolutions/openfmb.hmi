import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { CommandAction, PosString } from '../../../shared/hmi.constants'
import { DiagramData } from '../../../shared/models/userobject.model';

@Component({
  selector: 'app-switchgear-dialog',
  templateUrl: './switchgear-dialog.component.html',
  styleUrls: ['./switchgear-dialog.component.scss']
})
export class SwitchgearDialogComponent implements OnInit {  
  status: string;
  name: string;
  actionColor: string;
  actionText: string;
  actionEnabled: boolean = true;
  diagramId: string;
  mRID: string;
  diagramData: DiagramData;  
  hasDataMapped: boolean = false;    

  constructor(
    public dialogRef: MatDialogRef<SwitchgearDialogComponent>,    
    @Inject(MAT_DIALOG_DATA) public data: any
  ) { }

  ngOnInit() {                    
    this.diagramId = this.data.diagramId;   
    this.diagramData = this.data.diagramData;
    this.name = this.diagramData.name,
    this.mRID = this.diagramData.mRID;
    this.status = this.diagramData.tag;
    
    if (!this.diagramData.controlData || this.diagramData.controlData.length == 0) {      
      this.hasDataMapped = false;
    }
    else {
      this.hasDataMapped = true;
    }    
    
    if (this.status && this.status.toLowerCase() === PosString.open) {
      this.actionText = CommandAction.CLOSE;
      this.actionColor = "red";
    }
    else if (this.status && this.status.toLowerCase() === PosString.closed){
      this.actionText = CommandAction.OPEN;
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
