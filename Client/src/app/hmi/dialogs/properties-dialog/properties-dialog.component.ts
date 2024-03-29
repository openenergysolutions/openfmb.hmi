// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { DiagramData } from '../../../shared/models/userobject.model'

@Component({
  selector: 'app-properties-dialog',
  templateUrl: './properties-dialog.component.html',
  styleUrls: ['./properties-dialog.component.scss']
})
export class PropertiesDialogComponent implements OnInit {  
  label: string;
  name: string;
  description: string;
  showDescription: boolean = false;
  diagramId: string;
  mRID: string; 
  lastUpdate: string;
  hasLastUpdate: boolean = false;    

  constructor(
    public dialogRef: MatDialogRef<PropertiesDialogComponent>,    
    @Inject(MAT_DIALOG_DATA) public data: DiagramData
  ) { }

  ngOnInit() {        
    this.label = this.data.label;
    this.name = this.data.name,
    this.description = this.data.description;
    this.showDescription = this.description && this.description !== "";
    this.mRID = this.data.mRID;    
    this.diagramId = this.data.diagramId; 
    this.lastUpdate = this.data.lastUpdate;
    if (this.lastUpdate) {
      this.hasLastUpdate = true;
    }          
  }

  // save all grid item data
  onClose(): void {    
    this.dialogRef.close();
  }
  
  onMessageInspector(): void {
    window.open('/inspector?mrid=' + this.mRID, '_blank', 'toolbar=0,width=850,height=700');
    this.dialogRef.close({
      proceed: false      
    });
  }
}
