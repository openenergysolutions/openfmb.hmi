import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { DiagramData } from '../../../shared/models/userobject.model'

@Component({
  selector: 'app-properties-dialog',
  templateUrl: './properties-dialog.component.html',
  styleUrls: ['./properties-dialog.component.scss']
})
export class PropertiesDialogComponent implements OnInit {
  filterData: any;
  label: string;
  name: string;
  diagramId: string;
  mRID: string;    

  constructor(
    public dialogRef: MatDialogRef<PropertiesDialogComponent>,    
    @Inject(MAT_DIALOG_DATA) public data: DiagramData
  ) { }

  ngOnInit() {    
    this.filterData = this.data;
    this.label = this.data.label;
    this.name = this.data.name,
    this.mRID = this.data.mRID;    
    this.diagramId = this.data.diagramId;    
    
    this.dialogRef.updatePosition({
      top: `${this.filterData.top}px`,
      left: `${this.filterData.left}px`
    });     
  }

  // save all grid item data
  onClose(): void {    
    this.dialogRef.close();
  }  
}
