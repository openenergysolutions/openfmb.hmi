// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ButtonLoadingComponent } from './button-loading.component';

describe('ButtonLoadingComponent', () => {
  let component: ButtonLoadingComponent;
  let fixture: ComponentFixture<ButtonLoadingComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ButtonLoadingComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ButtonLoadingComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
