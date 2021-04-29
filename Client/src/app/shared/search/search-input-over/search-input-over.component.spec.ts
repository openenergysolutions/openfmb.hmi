// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { SearchInputOverComponent } from './search-input-over.component';

describe('SearchInputOverComponent', () => {
  let component: SearchInputOverComponent;
  let fixture: ComponentFixture<SearchInputOverComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ SearchInputOverComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(SearchInputOverComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
