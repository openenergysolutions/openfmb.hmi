// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, inject } from "@angular/core";
import { MatDialogRef, MAT_DIALOG_DATA } from "@angular/material/dialog";

@Component({
    selector: "app-generic-dialog",
    templateUrl: "./generic-dialog.component.html",
    styleUrls: ["./generic-dialog.component.scss"]
})
export class GenericDialogComponent implements OnInit {
  dialogRef = inject<MatDialogRef<GenericDialogComponent>>(MatDialogRef);
  data = inject(MAT_DIALOG_DATA);

  title: string;
  message: string;

  ngOnInit() {
    this.message = this.data.message;
    this.title = this.data.title;
  }

  // save all grid item data
  onClose(): void {
    this.dialogRef.close();
  }
}
