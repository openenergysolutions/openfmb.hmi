// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, inject } from "@angular/core";
import { MatDialogRef, MAT_DIALOG_DATA } from "@angular/material/dialog";
import {
  UntypedFormBuilder,
  Validators,
  UntypedFormGroup,
} from "@angular/forms";
import { Authorization } from "../../../shared/models/user.model";

@Component({
    selector: "app-dialogs",
    templateUrl: "./dialogs.component.html",
    styleUrls: ["./dialogs.component.scss"]
})
export class DialogsComponent implements OnInit {
  data = inject(MAT_DIALOG_DATA);
  dialogRef = inject<MatDialogRef<DialogsComponent>>(MatDialogRef);
  private fb = inject(UntypedFormBuilder);

  public itemForm: UntypedFormGroup;
  roles: any[];
  selectedRole = "";

  ngOnInit() {
    this.roles = [
      Authorization.authRoles.admin,
      Authorization.authRoles.engineer,
      Authorization.authRoles.viewer,
    ];
    this.buildItemForm(this.data.payload);
  }
  buildItemForm(item) {
    this.itemForm = this.fb.group({
      id: [item.id || ""],
      username: [item.username || "", Validators.required],
      displayname: [item.displayname || "", Validators.required],
      role: [item.role || "", Validators.required],
      pwd: [item.pwd || "", Validators.required],
    });
    this.selectedRole = item.role;
  }

  submit() {
    this.dialogRef.close(this.itemForm.value);
  }
}
