import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { FormBuilder, Validators, FormGroup } from '@angular/forms';


@Component({
  selector: 'app-dialogs',
  templateUrl: './dialogs.component.html',
  styleUrls: ['./dialogs.component.scss']
})
export class DialogsComponent implements OnInit {  
  public itemForm: FormGroup;  
  backgroundColor: any;

  constructor(
    @Inject(MAT_DIALOG_DATA) public data: any,
    public dialogRef: MatDialogRef<DialogsComponent>,
    private fb: FormBuilder,
  ) {}

  ngOnInit() {
    this.buildItemForm(this.data.payload);    
  }
  buildItemForm(item) {    
    this.itemForm = this.fb.group({
      diagramId: [item.diagramId || null],
      name: [item.name || '', Validators.required],
      description: [item.description || ''],
      location: [item.location || ''],
      backgroundColor: [item.backgroundColor || '']
    })
  }

  submit() {
    this.dialogRef.close(this.itemForm.value)
  }
}
