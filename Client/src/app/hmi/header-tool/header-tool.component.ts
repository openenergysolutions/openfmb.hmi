// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, EventEmitter, Output, ElementRef, ViewChild, Inject } from '@angular/core';
import { MatIconRegistry } from '@angular/material/icon';
import { DomSanitizer } from '@angular/platform-browser';
import { DesignerConstant } from './../../core/constants/designer-constant';
import { Store } from '@ngrx/store';
import { Observable } from 'rxjs';
import * as fromRoot from '../../store/reducers/index';
import * as designerActions from '../../store/actions/designer.actions';
import { CommunicationStatus } from '../../store/reducers/hmi.reducer';
import { NgxSpinnerService } from 'ngx-spinner';
import { AuthService } from '@auth0/auth0-angular';
import { DOCUMENT } from "@angular/common";

@Component({
  selector: 'app-header-tool',
  templateUrl: './header-tool.component.html',
  styleUrls: ['./header-tool.component.scss']
})
export class HeaderToolComponent implements OnInit {
  @ViewChild('fileInput', { static: true })
  fileInput: ElementRef;
  @Output() saveGraph = new EventEmitter();
  @Output() zoomGraph = new EventEmitter();
  @Output() exportGraph = new EventEmitter();
  @Output() loadGraph = new EventEmitter();
  @Output() runGraph = new EventEmitter();
  readonly DesignerConstant = DesignerConstant;
  selectedMode$: Observable<number>;
  commStatus$: Observable<number>;
  selectedConnectColor$: Observable<string>;
  isMoveSelected = false;
  autoTicks = false;
  disabled = false;
  invert = false;
  max = 2;
  min = 0.5;
  showTicks = false;
  step = 0.1;
  thumbLabel = false;
  value: number = 1;
  vertical = false;
  tickInterval = 1;
  commLost: boolean = false;

  constructor(
    private iconRegistry: MatIconRegistry,
    private sanitizer: DomSanitizer,
    private store: Store<fromRoot.State>,
    private spinner: NgxSpinnerService,
    public auth: AuthService,
    @Inject(DOCUMENT) private doc: Document
  ) {
    this.iconRegistry.addSvgIcon(
      'select-icon',
      this.sanitizer.bypassSecurityTrustResourceUrl('../../../assets/images/cursor.svg'));
    this.iconRegistry.addSvgIcon(
      'move-icon',
      this.sanitizer.bypassSecurityTrustResourceUrl('../../../assets/images/move.svg'));
    this.iconRegistry.addSvgIcon(
      'connect-icon',
      this.sanitizer.bypassSecurityTrustResourceUrl('../../../assets/images/connect.svg'));
  }

  ngOnInit() {
    this.selectedMode$ = this.store.select(state => state.designer.mode);
    this.selectedMode$.subscribe(
      x => this.isMoveSelected = x === 2,
      err => console.error('Observer got an error: ' + err),
      () => console.log('Observer got a complete notification')
    );
    this.selectedConnectColor$ = this.store.select(state => state.designer.connectColor);

    this.commStatus$ = this.store.select(state => state.hmi.status);
    this.commStatus$.subscribe(
      x => {
        this.commLost = x === CommunicationStatus.NOT_OK;
      },
      err => console.error('Observer got an error: ' + err),
      () => console.log('Observer got a complete notification')
    );
  }

  onModeSelect() {
    if (this.isMoveSelected) {
      this.store.dispatch(designerActions.selectMode({ mode: DesignerConstant.SELECT_MODE }));
    }
    else {
      this.store.dispatch(designerActions.selectMode({ mode: DesignerConstant.MOVE_MODE }));
    }
  }

  onColorSelect(connectColor: string) {
    this.store.dispatch(designerActions.selectColor({ connectColor }));
  }

  onSaveGraph() {
    this.saveGraph.emit();
  }

  onZoomGraph() {
    this.zoomGraph.emit(this.value);
  }

  zoomIn() {
    if (this.value < this.max) {
      this.value = (this.value * 10 + 1) / 10;
      this.zoomGraph.emit(this.value);
    }
  }

  zoomOut() {
    if (this.value > this.min) {
      this.value = (this.value * 10 - 1) / 10;
      this.zoomGraph.emit(this.value);
    }
  }

  onExportGraph() {
    this.exportGraph.emit();
  }

  onLoadGraph(file: File) {
    this.fileInput.nativeElement.value = '';
    this.spinner.show();
    this.loadGraph.emit(file);
  }

  onRunGraph() {
    this.runGraph.emit();
  }

  logout() {
    this.auth.logout({ returnTo: this.doc.location.origin });
  }
}
