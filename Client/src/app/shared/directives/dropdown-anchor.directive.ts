// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Directive, HostListener, Inject } from '@angular/core';
import { DropdownLinkDirective } from './dropdown-link.directive';

@Directive({
  selector: '[appDropdownToggle]'
})
export class DropdownAnchorDirective {

  protected navlink: DropdownLinkDirective;
  
    constructor( @Inject(DropdownLinkDirective) navlink: DropdownLinkDirective) {
      this.navlink = navlink;
    }
  
    @HostListener('click', ['$event'])
    onClick(e: any) {
      this.navlink.toggle();
    }
}
