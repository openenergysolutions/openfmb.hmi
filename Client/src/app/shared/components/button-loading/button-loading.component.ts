// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, Input } from "@angular/core";

@Component({
  selector: "button-loading",
  templateUrl: "./button-loading.component.html",
  styleUrls: ["./button-loading.component.scss"],
})
export class ButtonLoadingComponent {
  @Input() loading: boolean;
  @Input() btnClass: string;
  @Input() raised: boolean = true;
  @Input() loadingText = "Please wait";
  @Input() type: "button" | "submit" = "submit";
  @Input() color: "primary" | "accent" | "warn";

  constructor() {}
}
