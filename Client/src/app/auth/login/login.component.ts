// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit } from '@angular/core';
import { FormGroup, FormControl, Validators, FormBuilder } from '@angular/forms';
import * as fromRoot from '../../store/reducers/index';
import * as authActions from '../../store/actions/auth.actions';
import { Store } from '@ngrx/store';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss']
})
export class LoginComponent implements OnInit {
  loginForm: FormGroup;

  constructor(
    private fb: FormBuilder,
    private store: Store<fromRoot.State>
  ) {
    console.log("Create login component.");
  }

  ngOnInit() {
    this.initLoginForm();
  }

  initLoginForm() {
    this.loginForm = this.fb.group({
      username: new FormControl('', Validators.required),
      password: new FormControl('', Validators.required),
    });
  }

  login() {
    // stop here if form is invalid
    if (this.loginForm.invalid) {
      return;
    }
    // if valid made the backend request for validate.
    this.store.dispatch(authActions.login({ username: this.loginForm.get('username').value, password: this.loginForm.get('password').value }));
  }

}
