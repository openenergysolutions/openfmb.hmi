// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import {
  Directive,
  OnInit,
  OnDestroy,
  HostBinding,
  Input,
  HostListener
} from "@angular/core";
import { takeUntil } from "rxjs/operators";
import { Subject } from "rxjs";
import { MatchMediaService } from "../../services/match-media.service";
import { SidenavHelperService } from "./sidenav-helper.service";
import { MatSidenav } from "@angular/material/sidenav";
import { MediaObserver } from "@angular/flex-layout";

@Directive({
  selector: "[sidenavHelper]"
})
export class SidenavHelperDirective implements OnInit, OnDestroy {
  @HostBinding("class.is-open")
  isOpen: boolean;

  @Input("sidenavHelper")
  id: string;

  @Input("isOpen")
  isOpenBreakpoint: string;

  private unsubscribeAll: Subject<any>;

  constructor(
    private matchMediaService: MatchMediaService,
    private sidenavHelperService: SidenavHelperService,
    private matSidenav: MatSidenav,
    private mediaObserver: MediaObserver
  ) {
    // Set the default value
    this.isOpen = true;

    this.unsubscribeAll = new Subject();
  }

  ngOnInit(): void {
    this.sidenavHelperService.setSidenav(this.id, this.matSidenav);

    if (this.mediaObserver.isActive(this.isOpenBreakpoint)) {
      this.isOpen = true;
      this.matSidenav.mode = "side";
      this.matSidenav.toggle(true);
    } else {
      this.isOpen = false;
      this.matSidenav.mode = "over";
      this.matSidenav.toggle(false);
    }

    this.matchMediaService.onMediaChange
      .pipe(takeUntil(this.unsubscribeAll))
      .subscribe(() => {
        if (this.mediaObserver.isActive(this.isOpenBreakpoint)) {
          this.isOpen = true;
          this.matSidenav.mode = "side";
          this.matSidenav.toggle(true);
        } else {
          this.isOpen = false;
          this.matSidenav.mode = "over";
          this.matSidenav.toggle(false);
        }
      });
  }

  ngOnDestroy(): void {
    this.unsubscribeAll.next();
    this.unsubscribeAll.complete();
  }
}

@Directive({
  selector: "[sidenavToggler]"
})
export class SidenavTogglerDirective {
  @Input("sidenavToggler")
  public id: any;

  constructor(private sidenavHelperService: SidenavHelperService) {}

  @HostListener("click")
  onClick() {    
    this.sidenavHelperService.getSidenav(this.id).toggle();
  }
}
