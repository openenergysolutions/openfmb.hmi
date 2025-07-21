// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import {
  Directive,
  ElementRef,
  OnInit,
  Input,
  NgZone,
  SimpleChanges,
  OnChanges,
  ChangeDetectorRef,
} from "@angular/core";
import hljs from "highlight.js";
import { HttpClient } from "@angular/common/http";
import { UntilDestroy, untilDestroyed } from "@ngneat/until-destroy";

@UntilDestroy()
@Directive({
    host: {
        "[class.hljs]": "true",
        "[innerHTML]": "highlightedCode",
    },
    selector: "[hmiHighlight]",
    standalone: false
})
export class HighlightDirective implements OnInit, OnChanges {
  constructor(
    private el: ElementRef,
    private cdr: ChangeDetectorRef,
    private _zone: NgZone,
    private http: HttpClient,
  ) {}

  // Inner highlighted html
  highlightedCode: string;

  @Input() path: string;
  @Input("hmiHighlight") code: string;
  @Input() languages: string[];

  ngOnInit() {
    if (this.code) {
      this.highlightElement(this.code);
    }
    if (this.path) {
      this.highlightedCode = "Loading...";
      this.http
        .get(this.path, { responseType: "text" })
        .pipe(untilDestroyed(this))
        .subscribe((response) => {
          this.highlightElement(response, this.languages);
        });
    }
  }

  ngOnChanges(changes: SimpleChanges) {
    if (
      changes["code"] &&
      changes["code"].currentValue &&
      changes["code"].currentValue !== changes["code"].previousValue
    ) {
      this.highlightElement(this.code);
      // console.log('hljs on change', changes)
    }
  }

  highlightElement(code: string, _?: string[]) {
    this._zone.runOutsideAngular(() => {
      const res = hljs.highlightAuto(code);
      this.highlightedCode = res.value;
    });
  }
}
