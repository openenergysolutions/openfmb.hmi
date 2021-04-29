// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, AfterViewInit, Renderer2 } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { Router, NavigationEnd, ActivatedRoute, ActivatedRouteSnapshot } from '@angular/router';
import * as fromRoot from './store/reducers/index';
import * as authActions from './store/actions/auth.actions';
import { Store } from '@ngrx/store';

import { RoutePartsService } from "./shared/services/route-parts.service";
// import { ThemeService } from './shared/services/theme.service';

import { filter } from 'rxjs/operators';
// import { LayoutService } from './shared/services/layout.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit, AfterViewInit {
  appTitle = 'HMI';
  pageTitle = '';

  constructor(
    public title: Title, 
    private router: Router, 
    private activeRoute: ActivatedRoute,
    private routePartsService: RoutePartsService,
    private store: Store<fromRoot.State>
    // private themeService: ThemeService,
    // private layout: LayoutService,
    // private renderer: Renderer2
  ) { }

  ngOnInit() {
    this.changePageTitle();
  }
  ngAfterViewInit() {
  }
  changePageTitle() {
    this.router.events.pipe(filter(event => event instanceof NavigationEnd)).subscribe((routeChange) => {
      var routeParts = this.routePartsService.generateRouteParts(this.activeRoute.snapshot);
      if (!routeParts.length)
        return this.title.setTitle(this.appTitle);
      // Extract title from parts;
      this.pageTitle = routeParts
                      .reverse()
                      .map((part) => part.title )
                      .reduce((partA, partI) => {return `${partA} > ${partI}`});
      this.pageTitle += ` | ${this.appTitle}`;
      this.title.setTitle(this.pageTitle);
    });
  }
  
}
