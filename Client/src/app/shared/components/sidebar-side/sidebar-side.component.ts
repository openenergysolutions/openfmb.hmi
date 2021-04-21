import { Component, OnInit, OnDestroy, AfterViewInit } from "@angular/core";
import { NavigationService } from "../../../shared/services/navigation.service";
import { ThemeService } from "../../services/theme.service";
import { Subscription } from "rxjs";
import { ILayoutConf, LayoutService } from "../../../../app/shared/services/layout.service";
import { JwtAuthService } from "../../../../app/shared/services/auth/jwt-auth.service";
import { LocalStoreService } from "../../services/local-store.service";

@Component({
  selector: "app-sidebar-side",
  templateUrl: "./sidebar-side.component.html"
})
export class SidebarSideComponent implements OnInit, OnDestroy, AfterViewInit {
  public menuItems: any[] = [];
  public hasIconTypeMenuItem: boolean;
  public iconTypeMenuTitle: string;
  private menuItemsSub: Subscription;
  public layoutConf: ILayoutConf;
  public userDisplayName: string;

  constructor(
    private navService: NavigationService,
    public themeService: ThemeService,
    private layout: LayoutService,
    public jwtAuth: JwtAuthService,
    private ls: LocalStoreService
  ) {}

  ngOnInit() {
    this.iconTypeMenuTitle = this.navService.iconTypeMenuTitle;
    this.menuItemsSub = this.navService.menuItems$.subscribe(menuItem => {

      //this.menuItems = menuItem;
      for(var i = 0; i < menuItem.length; ++i) {
        if (menuItem[i].visible === true) {
          this.menuItems.push(menuItem[i]);
        }
      }
      //Checks item list has any icon type.
      this.hasIconTypeMenuItem = !!this.menuItems.filter(
        item => item.type === "icon"
      ).length;
    });
    this.layoutConf = this.layout.layoutConf;
    this.userDisplayName = this.jwtAuth.getUser()?.displayname;
  }
  ngAfterViewInit() {}
  ngOnDestroy() {
    if (this.menuItemsSub) {
      this.menuItemsSub.unsubscribe();
    }
  }
  toggleCollapse() {
    if (this.layoutConf.sidebarCompactToggle) {
      this.ls.setItem("sidebarCompactToggle", false);
      this.layout.publishLayoutChange({
        sidebarCompactToggle: false
      });
    } 
    else {
      this.ls.setItem("sidebarCompactToggle", true);
      this.layout.publishLayoutChange({          
        sidebarCompactToggle: true
      });
    }
  }
}
