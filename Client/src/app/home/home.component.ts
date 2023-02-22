// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnDestroy, OnInit } from '@angular/core';
import { Authorization } from '../shared/models/user.model';
import { AuthService } from "@auth0/auth0-angular";
import { AuthConstant } from '../core/constants/auth-constant';
import { Subscription } from 'rxjs';
import { parseJwt } from '../shared/helpers/utils';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit, OnDestroy {
  canEditDiagram: boolean = false;
  userSub: Subscription;
  constructor(private auth: AuthService) { }
  
  ngOnInit(): void {
    this.userSub = this.auth.user$.subscribe(user => {
      if (user) {
        this.auth.getAccessTokenSilently().toPromise().then(access_token => {
          const access_token_d = parseJwt(access_token);
          const roles = access_token_d.resource_access.gms.roles;
          this.canEditDiagram = Authorization.canEditDiagram(roles);
        });
      }
    })
  }

  ngOnDestroy() {
    if (this.userSub) {
      this.userSub.unsubscribe()
    }
  }

}
