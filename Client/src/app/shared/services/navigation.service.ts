// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { Authorization } from '../../shared/models/user.model';
import { AuthService } from '@auth0/auth0-angular';
import { AuthConstant } from '../../core/constants/auth-constant';
import { parseJwt } from '../helpers/utils';

interface IMenuItem {
  type: string; // Possible values: link/dropDown/icon/separator/extLink
  name?: string; // Used as display text for item and title for separator type
  state?: string; // Router state
  icon?: string; // Material icon name
  tooltip?: string; // Tooltip text
  disabled?: boolean; // If true, item will not be appeared in sidenav.
  sub?: IChildItem[]; // Dropdown items
  badges?: IBadge[];
  visible?: boolean;
}
interface IChildItem {
  type?: string;
  name: string; // Display text
  state?: string; // Router state
  icon?: string;
  sub?: IChildItem[];
}

interface IBadge {
  color: string; // primary/accent/warn/hex color codes(#fff000)
  value: string; // Display text
}

@Injectable()
export class NavigationService {

  iconMenu: IMenuItem[] = [
    {
      name: 'DIAGRAMS',
      type: 'link',
      tooltip: 'Diagrams',
      icon: 'dashboard',
      state: 'diagrams',
      visible: true
    },
    {
      name: 'DATA CONNECTION',
      type: 'link',
      tooltip: 'Data connection',
      icon: 'settings_remote',
      state: 'data-connect',
      visible: false, //Default off until authorization updates
    },
    {
      name: "SETTINGS",
      type: "dropDown",
      tooltip: "Settings",
      icon: "settings",
      state: "settings",
      visible: false, //Default off until authorization updates
      sub: [
        // TODO: Cleanup comments and console.logs
        // { name: "Users", state: "users" }, // Not used in OAuth
        { name: "Devices", state: "devices" }
      ]
    }
  ]
  // Icon menu TITLE at the very top of navigation.
  // This title will appear if any icon type item is present in menu.
  iconTypeMenuTitle = 'Frequently Accessed';
  // sets iconMenu as default;
  menuItems = new BehaviorSubject<IMenuItem[]>(this.iconMenu);
  // navigation component has subscribed to this Observable
  menuItems$ = this.menuItems.asObservable();

  constructor(public auth: AuthService) {
    this.auth.user$.subscribe(user => {
      if (user) {
        this.auth.getAccessTokenSilently({audience: "openfmb-hmi"}).toPromise().then(access_token => {
          const access_token_d = parseJwt(access_token);
          const roles = access_token_d.resource_access.gms.roles;
          this.iconMenu.filter(item => item.name === 'DATA CONNECTION')[0].visible = Authorization.canEditDiagram(roles);
          this.iconMenu.filter(item => item.name === 'SETTINGS')[0].visible = Authorization.canUpdateSettings(roles);
        })
      }
    });
  }

  // Customizer component uses this method to change menu.
  // You can remove this method and customizer component.
  // Or you can customize this method to supply different menu for
  // different user type.
  publishNavigationChange(menuType: string) {
    this.menuItems.next(this.iconMenu);
  }
}
