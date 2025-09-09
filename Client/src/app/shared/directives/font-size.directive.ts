// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Directive, ElementRef, OnInit, HostAttributeToken, inject } from "@angular/core";

@Directive({
    selector: "[fontSize]"
})
export class FontSizeDirective implements OnInit {
  fontSize = inject(new HostAttributeToken("fontSize"));
  private el = inject(ElementRef);

  ngOnInit() {
    this.el.nativeElement.fontSize = this.fontSize;
  }
}
