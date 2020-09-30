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
  constructor(
    @Inject(MAT_DIALOG_DATA) public data: any,
    public dialogRef: MatDialogRef<DialogsComponent>,
    private fb: FormBuilder,
  ) {}

  ngOnInit() {
    this.buildItemForm(this.data.payload)
  }
  buildItemForm(item) {
    this.itemForm = this.fb.group({
      userName: [item.userName || '', Validators.required],
      firstName: [item.firstName || '', Validators.required],
      lastName: [item.lastName || '', Validators.required],
      role: [item.role || '', Validators.required],
      password: [item.password || '', Validators.required]
    })
  }

  submit() {
    this.dialogRef.close(this.itemForm.value)
  }
}
