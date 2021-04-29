// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs/BehaviorSubject";
import { Observable } from "rxjs";

@Injectable({
  providedIn: "root"
})
export class SearchService {
  public searchTerm: BehaviorSubject<string> = new BehaviorSubject<string>("");
  public searchTerm$: Observable<string> = this.searchTerm.asObservable();

  constructor() {}
}
