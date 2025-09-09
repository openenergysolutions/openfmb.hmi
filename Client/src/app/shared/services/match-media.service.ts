// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable, inject } from "@angular/core";
import { MediaObserver, MediaChange } from "ngx-flexible-layout";
import { BehaviorSubject } from "rxjs";

@Injectable({
  providedIn: "root",
})
export class MatchMediaService {
  private mediaObserver = inject(MediaObserver);

  activeMediaQuery: string;
  onMediaChange: BehaviorSubject<string> = new BehaviorSubject<string>("");

  constructor() {
    this.activeMediaQuery = "";
    this.init();
  }

  private init(): void {
    this.mediaObserver.asObservable().subscribe((changes: MediaChange[]) => {
      changes.forEach((change) => {
        if (this.activeMediaQuery !== change.mqAlias) {
          this.activeMediaQuery = change.mqAlias;
          this.onMediaChange.next(change.mqAlias);
        }
      });
    });
  }
}
