// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { MatDialogRef, MAT_DIALOG_DATA } from "@angular/material/dialog";
import { Component, inject } from "@angular/core";

@Component({
    selector: "app-confirm",
    template: `<h1 matDialogTitle class="mb-05">{{ data.title }}</h1>
    <div mat-dialog-content class="mb-1">{{ data.message }}</div>
    <div mat-dialog-actions>
      <button
        type="button"
        mat-raised-button
        color="primary"
        (click)="dialogRef.close(true)"
      >
        OK
      </button>
      &nbsp;
      <span fxFlex></span>
      <button
        type="button"
        color="accent"
        mat-raised-button
        (click)="dialogRef.close(false)"
      >
        Cancel
      </button>
    </div>`
})
export class AppComfirmComponent {
  dialogRef = inject<MatDialogRef<AppComfirmComponent>>(MatDialogRef);
  data = inject(MAT_DIALOG_DATA);
}
