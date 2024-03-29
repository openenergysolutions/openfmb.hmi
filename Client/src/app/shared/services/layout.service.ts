// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { ThemeService } from "./theme.service";
import { LocalStoreService } from "./local-store.service";

export interface ILayoutConf {
  navigationPos?: string; // side, top
  sidebarStyle?: string; // full, compact, closed
  sidebarCompactToggle?: boolean; // sidebar expandable on hover
  sidebarColor?: string; // Sidebar background color
  dir?: string; // ltr, rtl
  isMobile?: boolean; // updated automatically
  useBreadcrumb?: boolean; // Breadcrumb enabled/disabled
  breadcrumb?: string; // simple, title
  topbarFixed?: boolean; // Fixed header
  footerFixed?: boolean; // Fixed Footer
  topbarColor?: string; // Header background color
  footerColor?: string // Header background color
  matTheme?: string; // material theme
  perfectScrollbar?: boolean;
}
export interface ILayoutChangeOptions {
  duration?: number;
  transitionClass?: boolean;
}
interface IAdjustScreenOptions {
  browserEvent?: any;
  route?: string;
}

@Injectable({
  providedIn: "root"
})
export class LayoutService {
  public layoutConf: ILayoutConf;  
  layoutConfSubject = null;
  layoutConf$ = null;
  isCompact: boolean;
  public isMobile: boolean;
  public currentRoute: string;
  public fullWidthRoutes = ["shop"];

  constructor(private themeService: ThemeService, private ls: LocalStoreService) {     
    this.isCompact = ls.getItem("sidebarCompactToggle") == null ? false : ls.getItem("sidebarCompactToggle");    
    this.setAppLayout(      
      {
        navigationPos: "side", // side, top
        sidebarStyle: this.isCompact ? "compact" : "full", // full, compact, closed
        sidebarColor: "slate", 
        sidebarCompactToggle: this.isCompact, // applied when "sidebarStyle" is "compact"
        dir: "ltr", // ltr, rtl
        useBreadcrumb: false,
        topbarFixed: false,
        footerFixed: false,
        topbarColor: "white", 
        footerColor: "slate",
        matTheme: "hmi-navy",
        breadcrumb: "simple", // simple, title
        perfectScrollbar: true
      }
    );    

    this.layoutConfSubject = new BehaviorSubject<ILayoutConf>(this.layoutConf);
    this. layoutConf$ = this.layoutConfSubject.asObservable();
  }

  setAppLayout(layoutConf: ILayoutConf) {
    this.layoutConf = { ...this.layoutConf, ...layoutConf };
    this.applyMatTheme(this.layoutConf.matTheme);
  }

  publishLayoutChange(lc: ILayoutConf, opt: ILayoutChangeOptions = {}) {    
    if (this.layoutConf.matTheme !== lc.matTheme && lc.matTheme) {
      this.themeService.changeTheme(this.layoutConf.matTheme, lc.matTheme);
    }
    this.layoutConf = Object.assign(this.layoutConf, lc);      
    this.layoutConfSubject.next(this.layoutConf);
    this.isCompact = this.ls.getItem("sidebarCompactToggle") == null ? false : this.ls.getItem("sidebarCompactToggle");    
  }

  applyMatTheme(theme) {
    this.themeService.applyMatTheme(theme);
  }

  adjustLayout(options: IAdjustScreenOptions = {}) {
    let sidebarStyle: string;
    this.isMobile = this.isSm();
    this.currentRoute = options.route || this.currentRoute;
    sidebarStyle = this.isMobile ? "closed" : this.isCompact ? "compact" : "full";

    if (this.currentRoute) {
      this.fullWidthRoutes.forEach(route => {
        if (this.currentRoute.indexOf(route) !== -1) {
          sidebarStyle = "closed";
        }
      });
    }

    this.publishLayoutChange({
      isMobile: this.isMobile,
      sidebarStyle: sidebarStyle
    });
  }
  isSm() {
    return window.matchMedia(`(max-width: 959px)`).matches;
  }
}
