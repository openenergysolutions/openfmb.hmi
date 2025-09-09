// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, HostListener, ChangeDetectorRef, OnDestroy, inject } from "@angular/core";
import {
  Router,
  NavigationEnd,
  RouteConfigLoadStart,
  RouteConfigLoadEnd,
  ResolveStart,
  ResolveEnd,
} from "@angular/router";
import { Subscription } from "rxjs";
import { TranslateService } from "@ngx-translate/core";
import { ThemeService } from "../../../services/theme.service";
import { LayoutService } from "../../../services/layout.service";
import { filter } from "rxjs/operators";
import { JwtAuthService } from "../../../services/auth/jwt-auth.service";

@Component({
    selector: "app-main-layout",
    templateUrl: "./main-layout.component.html"
})
export class MainLayoutComponent implements OnInit, OnDestroy {
  private router = inject(Router);
  translate = inject(TranslateService);
  themeService = inject(ThemeService);
  private layout = inject(LayoutService);
  private cdr = inject(ChangeDetectorRef);
  private jwtAuth = inject(JwtAuthService);

  public isModuleLoading: boolean = false;
  private moduleLoaderSub: Subscription;
  private layoutConfSub: Subscription;
  private routerEventSub: Subscription;

  public scrollConfig = {};
  public layoutConf: any = {};
  public adminContainerClasses: any = {};

  constructor() {
    const router = this.router;
    const translate = this.translate;

    // Check Auth Token is valid
    this.jwtAuth.checkTokenIsValid().subscribe();

    // Close sidenav after route change in mobile
    this.routerEventSub = router.events
      .pipe(filter((event) => event instanceof NavigationEnd))
      .subscribe((routeChange: NavigationEnd) => {
        this.layout.adjustLayout({ route: routeChange.url });
        this.scrollToTop();
      });

    // Translator init
    const browserLang: string = translate.getBrowserLang();
    translate.use(browserLang.match(/en|fr/) ? browserLang : "en");
  }

  ngOnInit() {
    // this.layoutConf = this.layout.layoutConf;
    this.layoutConfSub = this.layout.layoutConf$.subscribe((layoutConf) => {
      this.layoutConf = layoutConf;
      // console.log(this.layoutConf);

      this.adminContainerClasses = this.updateAdminContainerClasses(
        this.layoutConf,
      );
      this.cdr.markForCheck();
    });

    // FOR MODULE LOADER FLAG
    this.moduleLoaderSub = this.router.events.subscribe((event) => {
      if (
        event instanceof RouteConfigLoadStart ||
        event instanceof ResolveStart
      ) {
        this.isModuleLoading = true;
      }
      if (event instanceof RouteConfigLoadEnd || event instanceof ResolveEnd) {
        this.isModuleLoading = false;
      }
    });
  }
  @HostListener("window:resize", ["$event"])
  onResize(event) {
    this.layout.adjustLayout(event);
  }

  scrollToTop() {
    if (document) {
      setTimeout(() => {
        let element;
        if (this.layoutConf.topbarFixed) {
          element = <HTMLElement>(
            document.querySelector("#rightside-content-hold")
          );
        } else {
          element = <HTMLElement>document.querySelector("#main-content-wrap");
        }
        element.scrollTop = 0;
      });
    }
  }
  ngOnDestroy() {
    if (this.moduleLoaderSub) {
      this.moduleLoaderSub.unsubscribe();
    }
    if (this.layoutConfSub) {
      this.layoutConfSub.unsubscribe();
    }
    if (this.routerEventSub) {
      this.routerEventSub.unsubscribe();
    }
  }
  closeSidebar() {
    this.layout.publishLayoutChange({
      sidebarStyle: "closed",
    });
  }

  sidebarMouseenter(_) {
    // console.log(this.layoutConf);
    if (this.layoutConf.sidebarStyle === "compact") {
      this.layout.publishLayoutChange(
        { sidebarStyle: "full" },
        { transitionClass: true },
      );
    }
  }

  sidebarMouseleave(_) {
    // console.log(this.layoutConf);
    if (
      this.layoutConf.sidebarStyle === "full" &&
      this.layoutConf.sidebarCompactToggle
    ) {
      this.layout.publishLayoutChange(
        { sidebarStyle: "compact" },
        { transitionClass: true },
      );
    }
  }

  updateAdminContainerClasses(layoutConf) {
    return {
      "navigation-top": layoutConf.navigationPos === "top",
      "sidebar-full": layoutConf.sidebarStyle === "full",
      "sidebar-compact":
        layoutConf.sidebarStyle === "compact" &&
        layoutConf.navigationPos === "side",
      "compact-toggle-active": layoutConf.sidebarCompactToggle,
      "sidebar-compact-big":
        layoutConf.sidebarStyle === "compact-big" &&
        layoutConf.navigationPos === "side",
      "sidebar-opened":
        layoutConf.sidebarStyle !== "closed" &&
        layoutConf.navigationPos === "side",
      "sidebar-closed": layoutConf.sidebarStyle === "closed",
      "fixed-topbar":
        layoutConf.topbarFixed && layoutConf.navigationPos === "side",
    };
  }
}
