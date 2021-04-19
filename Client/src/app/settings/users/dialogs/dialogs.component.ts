import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { FormBuilder, Validators, FormGroup } from '@angular/forms';
import { Authorization } from '../../../shared/models/user.model';


@Component({
  selector: 'app-dialogs',
  templateUrl: './dialogs.component.html',
  styleUrls: ['./dialogs.component.scss']
})
export class DialogsComponent implements OnInit {  
  public itemForm: FormGroup;
  roles: any[];
  selectedRole = '';
  constructor(
    @Inject(MAT_DIALOG_DATA) public data: any,
    public dialogRef: MatDialogRef<DialogsComponent>,
    private fb: FormBuilder,
  ) {}

  ngOnInit() {
    this.roles = [ Authorization.authRoles.admin, Authorization.authRoles.engineer, Authorization.authRoles.viewer ];
    this.buildItemForm(this.data.payload);
    
  }
  buildItemForm(item) {
    this.itemForm = this.fb.group({
      id: [item.id || ''],
      username: [item.username || '', Validators.required],
      displayname: [item.displayname || '', Validators.required],      
      role: [item.role || '', Validators.required],
      pwd: [item.pwd || '', Validators.required]
    });
    this.selectedRole = item.role;
  }

  submit() {    
    this.dialogRef.close(this.itemForm.value)
  }
}
