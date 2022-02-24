// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { CommandAction, PosString, InternalTopic } from '../../../shared/hmi.constants'
import { DiagramData } from '../../../shared/models/userobject.model';
import { MatSnackBar } from '@angular/material/snack-bar';

@Component({
  selector: 'app-switchgear-dialog',
  templateUrl: './switchgear-dialog.component.html',
  styleUrls: ['./switchgear-dialog.component.scss']
})
export class SwitchgearDialogComponent implements OnInit {  
  status: string;
  name: string;
  description: string;
  showDescription: boolean = false;
  actionColor: string;
  actionText: string;
  actionEnabled: boolean = true;
  diagramId: string;
  mRID: string;
  diagramData: DiagramData;  
  hasDataMapped: boolean = false;
  lastUpdate: string;
  hasLastUpdate: boolean = false; 
  isCoordinatorActive: boolean = false;   

  constructor(
    public dialogRef: MatDialogRef<SwitchgearDialogComponent>,
    private snack: MatSnackBar,    
    @Inject(MAT_DIALOG_DATA) public data: any
  ) { }

  ngOnInit() {                       
    this.diagramId = this.data.diagramId;   
    this.diagramData = this.data.diagramData;
    this.name = this.diagramData.name,
    this.mRID = this.diagramData.mRID;
    this.description = this.diagramData.description;
    this.showDescription = this.description && this.description !== "";
    this.status = this.diagramData.tag;
    this.lastUpdate = this.diagramData.lastUpdate;
    this.isCoordinatorActive = this.data.isCoordinatorActive;
    if (this.lastUpdate) {
      this.hasLastUpdate = true;
    }
    
    if (!this.diagramData.controlData || this.diagramData.controlData.length == 0) {      
      this.hasDataMapped = false;
    }
    else {
      this.hasDataMapped = true;
    }    
    
    if (this.status && this.status.toLowerCase() === PosString.open) {
      this.actionText = this.getActionType(true); //CommandAction.CLOSE;
      this.actionColor = "red";
    }
    else if (this.status && this.status.toLowerCase() === PosString.closed){
      this.actionText = this.getActionType(false); // CommandAction.OPEN;
      this.actionColor = "green";
    }
    else {
      this.actionText = "INVALID";
      this.actionColor = "gray";
      this.actionEnabled = false;
    }
  }

  getActionType(close: boolean) : string {
    if (this.diagramData.controlData && this.diagramData.controlData.length > 0) {      
      var controlData = this.diagramData.controlData[0];
      console.log(controlData);
      if (controlData) {
        if (controlData.measurement == InternalTopic.isCoordinatorActive) {
          if (this.isCoordinatorActive == true) {
            this.snack.open('Coordination service is on Auto mode.  Please switch to Manual mode before any operation', 'OK', { duration: 15000 });
            return "";
          }
        }

        if (("" + controlData.path) == "PccControl") {
          return close ? CommandAction.CLOSE : CommandAction.OPEN;
        }
        if (("" + controlData.path).indexOf(".Pos.phs3.ctlVal") > 0) {
          return close ? CommandAction.CLOSE : CommandAction.OPEN;
        }
        if (("" + controlData.path).indexOf(".Pos.phsA.ctlVal") > 0) {
          return close ? CommandAction.CLOSE_PHSA : CommandAction.OPEN_PHSA;
        }
        if (("" + controlData.path).indexOf(".Pos.phsB.ctlVal") > 0) {
          return close ? CommandAction.CLOSE_PHSB : CommandAction.OPEN_PHSB;
        }
        if (("" + controlData.path).indexOf(".Pos.phsC.ctlVal") > 0) {
          return close ? CommandAction.CLOSE_PHSC : CommandAction.OPEN_PHSC;
        }
        
      }
    } else {
      this.hasDataMapped = false;
      this.snack.open('Invalid data mapping.  Mapping to DbPosKind is required.', 'OK', { duration: 4000 });
    }
    return "";
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

  onMessageInspector(): void {
    window.open('/inspector?mrid=' + this.mRID, '_blank', 'toolbar=0,width=850,height=700');
    this.dialogRef.close({
      proceed: false      
    });
  }
}
