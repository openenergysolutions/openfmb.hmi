import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { FormBuilder, Validators, FormGroup } from '@angular/forms';

@Component({
  selector: 'app-devicedialogs',
  templateUrl: './devicedialogs.component.html',
  styleUrls: ['./devicedialogs.component.scss']
})
export class DeviceDialogsComponent implements OnInit {  
  public itemForm: FormGroup; 
  constructor(
    @Inject(MAT_DIALOG_DATA) public data: any,
    public dialogRef: MatDialogRef<DeviceDialogsComponent>,
    private fb: FormBuilder,
  ) {}

  ngOnInit() {   
    this.buildItemForm(this.data.payload);
    
  }
  buildItemForm(item) {
    this.itemForm = this.fb.group({
      mrid: [item.mrid || '', Validators.required],
      name: [item.name || '', Validators.required]      
    });   
  }

  submit() {
    this.dialogRef.close(this.itemForm.value)
  }
}
