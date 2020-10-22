import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'app-generic-dialog',
  templateUrl: './generic-dialog.component.html',
  styleUrls: ['./generic-dialog.component.scss']
})
export class GenericDialogComponent implements OnInit {
  title: String;
  message: String; 

  constructor(
    public dialogRef: MatDialogRef<GenericDialogComponent>,    
    @Inject(MAT_DIALOG_DATA) public data: any
  ) { }

  ngOnInit() {    
    this.message = this.data.message;
    this.title = this.data.title;               
  }

  // save all grid item data
  onClose(): void {    
    this.dialogRef.close();
  }  
}
