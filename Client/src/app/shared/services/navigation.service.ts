import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { JwtAuthService } from "../../../app/shared/services/auth/jwt-auth.service";
import { Authorization } from '../../shared/models/user.model';

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
  userRole: string = this.jwtService.getUserRole();

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
      visible: Authorization.canEditDiagram(this.userRole)
    },
    {
      name: "SETTINGS",
      type: "dropDown",
      tooltip: "Settings",
      icon: "settings",
      state: "settings",
      visible: Authorization.canUpdateSettings(this.userRole),
      sub: [        
        { name: "Users", state: "users" }              
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

  constructor(private jwtService : JwtAuthService) {}  

  // Customizer component uses this method to change menu.
  // You can remove this method and customizer component.
  // Or you can customize this method to supply different menu for
  // different user type.
  publishNavigationChange(menuType: string) {
    this.menuItems.next(this.iconMenu);
  }
}
