// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import {
  Component,
  OnInit,
  ViewChild,
  ElementRef,
  AfterViewInit,
  Renderer2,
  TemplateRef,
  HostListener,
  OnDestroy
} from '@angular/core';
import { DesignerConstant } from './../core/constants/designer-constant';
import { mxgraph, mxgraphFactory } from 'ts-mxgraph';
import { Store } from '@ngrx/store';
import { MatDialog } from '@angular/material/dialog';
import { MatSnackBar } from '@angular/material/snack-bar';
import { PropertiesDialogComponent } from './dialogs/properties-dialog/properties-dialog.component';
import { SwitchgearDialogComponent } from './dialogs/switchgear-dialog/switchgear-dialog.component';
import { RegulatorDialogComponent } from './dialogs/regulator-dialog/regulator-dialog.component';
import { ControlDialogComponent } from './dialogs/control-dialog/control-dialog.component'
import { GenericDialogComponent } from './dialogs/generic-dialog/generic-dialog.component'
import { NgxSpinnerService } from 'ngx-spinner';
import toolbarItemsData from '../../assets/json/toolbar.json';
import * as fromRoot from '../store/reducers/index';
import { WebSocketService } from '../core/services/web-socket.service';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { v4 as uuidv4 } from 'uuid';
import { ActivatedRoute } from '@angular/router'
import { DiagramsService } from '../shared/services/diagrams.service';
import { Diagram } from '../shared/models/diagram.model';
import { DiagramData } from '../shared/models/userobject.model'
import { ButtonFunction, CommandAction, Helpers } from '../shared/hmi.constants'
import { Hmi, Symbol } from '../shared/hmi.constants'
import { Topic, UpdateData } from '../shared/models/topic.model'
import { Command } from '../shared/models/command.model';
import { JwtAuthService } from '../shared/services/auth/jwt-auth.service';

const {
  mxGraph,
  mxToolbar,
  mxUtils,
  mxRubberband,
  mxEdgeHandler,
  mxPoint,
  mxConstraintHandler,
  mxImage,
  mxConstants,
  mxEvent,
  mxPanningManager,
  mxGraphView,
  mxLog,
  mxConnectionConstraint,
  mxCodec,
  mxKeyHandler
} = mxgraphFactory({
  mxLoadResources: false,
  mxLoadStylesheets: false,
});

@Component({
  selector: 'app-hmi',
  templateUrl: './hmi.component.html',
  styleUrls: ['../designer/designer.component.scss']
})
export class HmiComponent implements OnInit, AfterViewInit, OnDestroy {
  @ViewChild('graphContainer', { static: false }) graphContainer: ElementRef;
  @ViewChild('toolbarContainer', { static: false }) toolbarContainer: ElementRef;
  DESIGNER_CONST = DesignerConstant;
  graph: mxgraph.mxGraph;
  toolbar: mxgraph.mxToolbar;
  mode = this.DESIGNER_CONST.CONNECT_MODE;
  canvas: any;
  toolbarItems = toolbarItemsData;
  sessionId = '';
  gridData = {
    scale: 0,
    gridSize: 0,
    translationPoint: new mxPoint(),
    width: 0,
    height: 0
  };
  diagramId: string = null;
  currentDiagram: Diagram;
  showingLostConnection: boolean = false;  

  private destroy$ = new Subject();

  private interval;

  constructor(
    private renderer: Renderer2,
    private store: Store<fromRoot.State>,
    public dialog: MatDialog,
    private spinner: NgxSpinnerService,
    private wsService: WebSocketService,
    private snack: MatSnackBar,
    private router : ActivatedRoute,
    private diagramService: DiagramsService,
    private jwtAuth: JwtAuthService
  ) {
    // Check Auth Token is valid
    this.jwtAuth.checkTokenIsValid().subscribe();
    
    this.router.queryParams.subscribe(params => {
      this.diagramId = params['id'];      
    });
  }

  ngOnInit() {
  }

  ngAfterViewInit() {
    
    // initialize graph.
    this.graphInit();

    // load default styles
    this.defaultGraphStyles();

    // load graph
    if (this.diagramId) {
      this.loadGraphFromServer(this.diagramId);      
    }

    this.sessionId = uuidv4();    

    this.connect(this.sessionId);
  }

  // init graph.
  private graphInit() {    
    mxEvent.disableContextMenu(this.graphContainer.nativeElement);
    mxConstraintHandler.prototype.pointImage = new mxImage('../../assets/images/dot.gif', 10, 10);        
    this.graph = new mxGraph(this.graphContainer.nativeElement);
    this.graph.setPortsEnabled(false);
    this.graph.graphHandler.scaleGrid = true;
    this.graph.gridSize = 40;
    this.graph.multigraph = false;
    this.graph.setAllowDanglingEdges(false);
    this.graph.isHtmlLabel = (cell) => {
      return !this.graph.isSwimlane(cell);
    }; 
    
    this.dialog.closeAll();
    this.graph.setPanning(false);
    this.graph.setConnectable(false);
    this.graph.getSelectionModel().clear();
    this.graph.setCellsSelectable(false);
    this.graph.setDropEnabled(false);
    this.graph.setCellsMovable(false);

    // Open popup in double click.
    this.graph.dblClick = (evt: MouseEvent, cell: mxgraph.mxCell) => {

      if (this.showingLostConnection) {
        return;
      }

      const currentCellData = this.graph.model.getValue(cell).userObject;    
      if (
        this.graph.isEnabled() &&
        !mxEvent.isConsumed(evt) &&
        cell != null &&
        this.graph.isCellEditable(cell)
      ) { 
        
        const centerX = window.innerWidth / 2; 
        const centerY = window.innerHeight / 2;          
        const x = evt.offsetX;
        const y = evt.offsetY;
        
        if (Hmi.isSwitchgear(currentCellData.type)) {                             
          this.openSwitchgearDialog(x < centerX ? x + cell.getGeometry().width + 250 : x, y - 100 > centerY ? centerY : y, cell);
        }
        else if (Hmi.isVoltageRegulator(currentCellData.type)) {
          this.openVoltageRegulatorDialog(x < centerX ? x + cell.getGeometry().width + 250 : x, y - 100 > centerY ? centerY : y, cell);
        }
        else if (currentCellData.type === Symbol.measureBox || (evt.target as HTMLElement).tagName === 'image' || (evt.target as HTMLElement).tagName === 'path') {                   
          this.openDialog(x < centerX ? x + cell.getGeometry().width + 250 : x, y - 100 > centerY ? centerY : y, cell);                    
        }        
        else if (currentCellData.type === Symbol.setPointButton) {          
          this.openControlDialog(x < centerX ? x + cell.getGeometry().width + 250 : x, y - 100 > centerY ? centerY : y, cell);
        }
        else if (currentCellData.type === Symbol.statusIndicator) {
          this.openControlDialog(x < centerX ? x + cell.getGeometry().width + 250 : x, y - 100 > centerY ? centerY : y, cell);
        }
        else if (currentCellData.type === Symbol.button && currentCellData.func === ButtonFunction.command) {
          this.openControlDialog(x < centerX ? x + cell.getGeometry().width + 250 : x, y - 100 > centerY ? centerY : y, cell);
        }
      } 
      else {
        this.graph.view.setTranslate(0, 0);
      }
      mxEvent.consume(evt);
    };

    // close popup on empty graph area click.
    this.graph.addListener(mxEvent.CLICK, (evt: Event) => {
      if (!this.showingLostConnection) {
        this.dialog.closeAll();
      }
      mxEvent.consume(evt);
    });

    this.graph.getModel().addListener(mxEvent.CHANGE, (evt: Event) => {     
      // register with backend
      this.register();          
    });

    const rubberband = new mxRubberband(this.graph);

    // disable cell connection.
    this.graph.connectionHandler.isConnectableCell = () => {
      return false;
    };

    mxEdgeHandler.prototype.isConnectableCell = (cell) => {
      return this.graph.connectionHandler.isConnectableCell(cell);
    };

    this.graph.view.getTerminalPort = (state, terminal, source) => {
      return terminal;
    };

    const cellsMoved = this.graph.cellsMoved;
    this.graph.cellsMoved = (cells, dx, dy, isconnect, constrain, extend) => {
      if (dx && cells[0].value.userObject.left) {
        dx = dx + cells[0].value.userObject.left
      }

      if (dy && cells[0].value.userObject.top) {
        dy = dy + cells[0].value.userObject.top
      }

      cellsMoved.apply(this.graph, [cells, dx, dy, isconnect, constrain, extend]);
    };
   
    // sets label and data fields to grid item
    this.graph.convertValueToString = (cell) => {      
      const cellValue = this.graph.model.getValue(cell);
      
      if (cellValue && cellValue.userObject) {
        const userObject = cellValue.userObject;
        const displayData = cellValue.userObject.displayData;
        const controlData = cellValue.userObject.controlData;
        
        // Returns a DOM for the label
        const wrapper = this.renderer.createElement('div');
        this.renderer.addClass(wrapper, 'component-label');        
        this.renderer.setAttribute(wrapper, 'cell-id', cell.id);

        // override 'display-data-container' class
        var style = '';
        if (userObject.fontSize) {
          style += 'font-size:' + userObject.fontSize + 'px;';
        }
        if (userObject.foreColor) {
          style += 'color: ' + userObject.foreColor + ';';
        }
        if (userObject.backgroundColor) {
          style += 'background-color: ' + userObject.backgroundColor + ';';
        }
        if (userObject.containerWidth) {
          style += 'width: ' + userObject.containerWidth + 'px;';
        }
        if (userObject.containerHeight) {
          style += 'height: ' + userObject.containerHeight + 'px;';
        }
        
        if (userObject.type === Symbol.measureBox) {          
          if (displayData) {
            const data = this.renderer.createElement('div');
            this.renderer.addClass(data, 'display-data-container');            

            if (style != '') {
              this.renderer.setAttribute(data, "style", style);
            }
            
            const titleContainer = this.renderer.createElement('div');
            const titleText = this.renderer.createText(userObject.label ? userObject.label : userObject.deviceTypeMapping ? userObject.deviceTypeMapping : "");
            this.renderer.addClass(titleContainer, 'data-title');
            this.renderer.appendChild(titleContainer, titleText);
            this.renderer.appendChild(data, titleContainer);
            
            displayData.forEach(elem => {
              const fieldItem = this.renderer.createElement('div');
              const fieldItemLabel = this.renderer.createElement('span');
              const fieldItemText = this.renderer.createElement('span');
              const fieldItemValue = this.renderer.createElement('span');
              this.renderer.addClass(fieldItemValue, elem.value); 
              this.renderer.setAttribute(fieldItemValue, 'mrid', userObject.mRID);
              this.renderer.setAttribute(fieldItemValue, 'path', elem.path);          
              const label = this.renderer.createText(elem.label);
              let text = '00';
              if (elem.label === 'State') {
                text = '';
                this.renderer.addClass(fieldItemValue, 'field-item-value-state');
              } else if (elem.label === 'Status') {
                text = '';
                this.renderer.addClass(fieldItemValue, 'field-item-value-status');
              } else if (elem.label === 'Mode') {
                text = '';
                this.renderer.addClass(fieldItemValue, 'field-item-value-mode');
              }
              const value = this.renderer.createText(text);
              this.renderer.addClass(fieldItem, 'field-item');
              this.renderer.addClass(fieldItemLabel, 'field-item-label');
              this.renderer.addClass(fieldItemValue, 'field-item-value');
              this.renderer.appendChild(fieldItemLabel, label);
              this.renderer.appendChild(fieldItemValue, value);
              this.renderer.appendChild(fieldItemText, fieldItemValue);
              this.renderer.setAttribute(fieldItemValue, 'cell-id', cell.id);
              if(elem.measurement) {
                const fieldItemMeasurement = this.renderer.createElement('span');
                const measurementText = this.renderer.createText(' ' + elem.measurement);
                this.renderer.appendChild(fieldItemMeasurement, measurementText);
                this.renderer.appendChild(fieldItemText, fieldItemMeasurement);
              }
              this.renderer.appendChild(fieldItem, fieldItemLabel);
              this.renderer.appendChild(fieldItem, fieldItemText);
              this.renderer.appendChild(data, fieldItem);
            });
            this.renderer.appendChild(wrapper, data);
          }
        }
        else if (userObject.type === Symbol.label) {                  
          var style = '';
          if (userObject.fontSize) {
            style += 'font-size:' + userObject.fontSize + 'px;';
          }
          if (userObject.foreColor) {
            style += 'color: ' + userObject.foreColor + ';';
          }          

          const span = this.renderer.createElement('span');
          this.renderer.addClass(span, 'label-text');

          if (style != '') {
            this.renderer.setAttribute(span, "style", style);
          }
          const labelText = this.renderer.createText(userObject.label);
          this.renderer.appendChild(span, labelText);          

          return span;       
        }
        else if (userObject.type === Symbol.button || userObject.type === Symbol.setPointButton) {     
          var style = '';
          if (userObject.fontSize) {
            style += 'font-size:' + userObject.fontSize + 'px;';
          }
          if (userObject.foreColor) {
            style += 'color: ' + userObject.foreColor + ';';
          }                 

          const span = this.renderer.createElement('span');
          this.renderer.addClass(span, 'button-text');
          this.renderer.setAttribute(span, 'mrid', userObject.mRID);
          this.renderer.setAttribute(span, 'cell-id', cell.id);

          if (userObject.func === ButtonFunction.link) {
            if (userObject.linkData) {
              if (userObject.linkData.diagramId) {
                var target = userObject.linkData.target ? userObject.linkData.target : '_blank';
                this.renderer.setAttribute(span, 'ondblclick', "navigateToDiagram('" + userObject.linkData.diagramId + "', '" + target + "')");
              }
            }
          }
          
          if (style != '') {
            this.renderer.setAttribute(span, "style", style);
          }
          const labelText = this.renderer.createText(userObject.label);
          this.renderer.appendChild(span, labelText);

          return span;
        }
        else if (userObject.type === Symbol.statusIndicator) {
          var style = '';
          if (userObject.fontSize) {
            style += 'font-size:' + userObject.fontSize + 'px;';
          }
          if (userObject.foreColor) {
            style += 'color: ' + userObject.foreColor + ';';
          }                           

          const span = this.renderer.createElement('span');
          this.renderer.addClass(span, 'button-text');
          this.renderer.setAttribute(span, 'mrid', userObject.mRID);
          this.renderer.setAttribute(span, 'cell-id', cell.id);
          
          if (displayData) {
            displayData.forEach(elem => {
              this.renderer.setAttribute(span, 'path', elem.path);
              this.renderer.setAttribute(span, 'obj-type', userObject.type);              
            });
          }
          
          if (style != '') {
            this.renderer.setAttribute(span, "style", style);
          }     
          
          // status 
          const img = this.renderer.createElement('img');
          this.renderer.setAttribute(img, 'src', 'assets/images/gray.svg');
          this.renderer.setAttribute(img, 'style', 'width:20px;height:20px;vertical-align:middle;padding-right:10px;');

          this.renderer.appendChild(span, img);

          const labelText = this.renderer.createText(userObject.label);
          this.renderer.appendChild(span, labelText);
          
          return span;
        }
        else {
          if (userObject.label) {
            const label = this.renderer.createElement('div');
            const labelText = this.renderer.createText(userObject.label);
            this.renderer.addClass(label, 'label');
            this.renderer.appendChild(label, labelText);
            this.renderer.appendChild(wrapper, label);
          }
          if (displayData && displayData.length > 0) {
            // write mrid to the wraper
            this.renderer.setAttribute(wrapper, 'svg-id', cell.id);
            this.renderer.setAttribute(wrapper, 'mrid', userObject.mRID);            
            
            displayData.forEach(elem => {
              const field = this.renderer.createElement('span');
              this.renderer.setAttribute(field, 'svg-id', cell.id);
              this.renderer.setAttribute(field, 'mrid', userObject.mRID);
              this.renderer.setAttribute(field, 'path', elem.path); 
              this.renderer.setAttribute(field, 'obj-type', userObject.type);
              this.renderer.appendChild(wrapper, field);
            });                        
          } 
          
          if (Hmi.isSwitchgear(userObject.type)) {
            var state = this.graph.view.getState(cell, false);
            if (state) {
              var node = state.shape.node;
              var image = node.getElementsByTagName("image")[0];
              var ref = image.getAttribute('href');
              if (!ref) {
                ref = image.getAttribute('xlink:href');
                image.removeAttribute('xlink:href');
              }
              const baseToolbarImagePath = '../../assets/images/toolbar/'; 
              image.setAttribute("href", baseToolbarImagePath + userObject.type + '-invalid.svg');
            }
          }
        }        

        return wrapper;
      }      
      return '';
    };    

    // This function trigger when cell label is changed.
    const cellLabelChanged = this.graph.cellLabelChanged;
    this.graph.cellLabelChanged = (cell, newValue, autoSize) => {
      const userObject = this.graph.model.getValue(cell).userObject;
      if (userObject) {
        // Clones the value for correct undo/redo
        const elt = cell.value.obj.cloneNode(true);
        elt.setAttribute('label', newValue);
        newValue = elt;
      }
      cellLabelChanged.apply(this, [cell, newValue, autoSize]);
    };

    // get all available connection ports
    this.graph.getAllConnectionConstraints = (terminal, source) => {
      if (terminal != null && terminal.shape != null &&
        terminal.shape.stencil != null) {
        // for stencils with existing constraints...
        if (terminal.shape.stencil != null) {
          return terminal.shape.stencil.constraints;
        }
      } else if (terminal != null && this.graph.getModel().isVertex(terminal.cell)) {
        if (terminal.shape != null) {
          const ports = this.graph.getModel().getValue(terminal.cell).ports;
          const cstrs = new Array();
          for (const id in ports) {
            if (id) {
              const port = ports[id];
              const cstr = new mxConnectionConstraint(new mxPoint(port.x, port.y), port.perimeter);
              cstrs.push(cstr);
            }
          }
          return cstrs;
        }
      }
      return null;
    };

    // enable graph pan.
    this.graph.createPanningManager = () => {
      const pm = new mxPanningManager(this.graph);
      pm.border = 30;
      return pm;
    };

    // enable right click popup.
    this.graph.popupMenuHandler.factoryMethod = (menu: mxgraph.mxPopupMenu, cell: mxgraph.mxCell, evt: Event) => {
      if (this.mode === this.DESIGNER_CONST.SELECT_MODE) {
        if (cell && cell.vertex) {
          menu.addItem('Remove', null, () => {
            this.graph.removeCells();
          });
        } else if (cell && cell.edge) {
          menu.addItem('Remove', null, () => {
            this.graph.removeCells();
          });
        }
      }
    };

    const keyHandler = new mxKeyHandler(this.graph);
    keyHandler.bindKey(46, (evt: Event) => {
      if (this.graph.isEnabled()) {
        this.graph.removeCells();
      }
    });

    try {
      this.graph.getModel().beginUpdate();
      this.graph.getStylesheet().getDefaultEdgeStyle().edgeStyle = 'orthogonalEdgeStyle';
      this.graph.getStylesheet().getDefaultEdgeStyle().endArrow = 'none';
    } finally {
      this.graph.getModel().endUpdate();
    }
  }  

  private isMeasureBox(cell: mxgraph.mxCell) {
    if (cell.value != null && cell.value.userObject != null) {
      if (cell.value.userObject.type != null && cell.value.userObject.type == "measure-box") {
        return true;
      }
    }
    return false; 
  }  

  // set default styles for graph.
  private defaultGraphStyles() {
    // Changes some default colors
    mxConstants.HANDLE_FILLCOLOR = '#99ccff';
    mxConstants.HANDLE_STROKECOLOR = '#0088cf';
    mxConstants.VERTEX_SELECTION_STROKEWIDTH = 2;
    mxConstants.VERTEX_SELECTION_DASHED = false;
    mxConstants.VERTEX_SELECTION_COLOR = '#00a8ff';

    const vertexStyle = this.graph.getStylesheet().getDefaultVertexStyle();
    vertexStyle[mxConstants.STYLE_VERTICAL_ALIGN] = 'bottom';

    const edgeStyle = this.graph.getStylesheet().getDefaultEdgeStyle();
    edgeStyle[mxConstants.STYLE_STROKEWIDTH] = '2';
  }

  // open property popup
  openDialog(x: number, y: number, cell: mxgraph.mxCell): void {
    const currentCellData = this.graph.model.getValue(cell).userObject;
    this.dialog.closeAll();
    const filterData : DiagramData = {
      top: y,
      left: x,
      diagramId: this.diagramId,
      label: currentCellData.label,
      name: currentCellData.name,
      displayData: currentCellData.displayData,
      controlData: currentCellData.controlData,
      mRID: currentCellData.mRID,
      fontSize: currentCellData.fontSize,
      containerWidth: currentCellData.containerWidth,
      containerHeight: currentCellData.containerHeight,
      foreColor: currentCellData.foreColor,
      backgroundColor: currentCellData.backgroundColor,      
      deviceTypeMapping: currentCellData.deviceTypeMapping,
      lastUpdate: currentCellData.lastUpdate
    };
    const dialogRef = this.dialog.open(PropertiesDialogComponent, {
      width: '355px',
      data: filterData,
      hasBackdrop: false,
      panelClass: 'filter-popup',
      autoFocus: true,
      closeOnNavigation: true
    });

    dialogRef.afterClosed().pipe(takeUntil(this.destroy$)).subscribe(result => {
      if (result) {        
        // do nothing
      }
    });
  }    

  openSwitchgearDialog(x: number, y: number, cell: mxgraph.mxCell) : void {
    const currentCellData = this.graph.model.getValue(cell).userObject;
    this.dialog.closeAll();
    const filterData = {
      top: y,
      left: x,
      diagramId: this.diagramId,
      diagramData: currentCellData
    };
    const dialogRef = this.dialog.open(SwitchgearDialogComponent, {
      width: '355px',
      data: filterData,
      hasBackdrop: false,
      panelClass: 'filter-popup',
      autoFocus: true,
      closeOnNavigation: true
    });

    dialogRef.afterClosed().pipe(takeUntil(this.destroy$)).subscribe(result => {
      if (result && result.proceed) {                
        this.sendCommand(currentCellData, result.action);
      }
    });
  } 
  
  openVoltageRegulatorDialog(x: number, y: number, cell: mxgraph.mxCell) : void {
    const currentCellData = this.graph.model.getValue(cell).userObject;
    this.dialog.closeAll();
    const filterData = {
      top: y,
      left: x,
      diagramId: this.diagramId,
      diagramData: currentCellData
    };
    const dialogRef = this.dialog.open(RegulatorDialogComponent, {
      width: '355px',
      data: filterData,
      hasBackdrop: false,
      panelClass: 'filter-popup',
      autoFocus: true,
      closeOnNavigation: true
    });

    dialogRef.afterClosed().pipe(takeUntil(this.destroy$)).subscribe(result => {
      if (result && result.proceed) {                
        this.sendTapChangerCommand(currentCellData, result.action, result.path);
      }
    });
  }

  openControlDialog(x: number, y: number, cell: mxgraph.mxCell): void {
    const currentCellData = this.graph.model.getValue(cell).userObject;
    this.dialog.closeAll();
    const filterData = {
      top: y,
      left: x,
      diagramId: this.diagramId,      
      diagramData: currentCellData
    };
    const dialogRef = this.dialog.open(ControlDialogComponent, {
      width: '355px',
      data: filterData,
      hasBackdrop: false,
      panelClass: 'filter-popup',
      autoFocus: true,
      closeOnNavigation: true
    });

    dialogRef.afterClosed().pipe(takeUntil(this.destroy$)).subscribe(result => {
      if (result && result.proceed) {                
        this.sendCommand(currentCellData, result.action, result.value);
      }
    });
  }

  showLostConnection(): void {    
    this.dialog.closeAll();
    const filterData = {
      title: "OpenFMB HMI",
      message: "Connection to server has lost!"
    };
    this.showingLostConnection = true;
    const dialogRef = this.dialog.open(GenericDialogComponent, {
      width: '355px',
      data: filterData,
      hasBackdrop: false,
      panelClass: 'filter-popup',
      autoFocus: true,
      closeOnNavigation: true
    });

    dialogRef.afterClosed().pipe(takeUntil(this.destroy$)).subscribe(result => {
      this.showingLostConnection = false;
    });
  }

  connect(sessionId: string) {
    this.wsService.connect(sessionId);
    this.wsService.wsConnection$
      .subscribe(
        (msg) => {
          if (msg) {
            if (this.showingLostConnection) {
              this.showingLostConnection = false;
              this.dialog.closeAll();
            }
            this.register();
          }
          else {
            //this.snack.open('Connection to server has lost.', 'OK', { duration: 4000 });                       
            this.showLostConnection();
          }
        },
        (error) => {
          console.log(error);
        }
      );
    this.wsService.wsMessages$ 
      .subscribe(
        (message) => {
          this.onReceivedMessage(message);
        },
        (error) => {
          console.log(error);
        }
      );
  }

  onReceivedMessage(message: any)
  {
    const baseToolbarImagePath = '../../assets/images/toolbar/';
    const baseImagePath = '../../assets/images/';
    const domElement = document.querySelectorAll('span[mrid]');

    const ts = Helpers.currentTimestamp();

    if (domElement.length > 0) {
      for(let update of message.updates) {            
        for(let i = 0; i < domElement.length; ++i) {
          if (update.topic?.mrid === domElement[i].getAttribute('mrid') && update.topic?.name === domElement[i].getAttribute('path')) {
            // check if there is 'svg-id' reference
            const svgId = domElement[i].getAttribute('svg-id');
            var objType = domElement[i].getAttribute('obj-type');
            if (svgId) {  // actual symbol
              try {
                if (Hmi.isSwitchgear(objType)) {
                  var cell = this.graph.getModel().getCell(svgId);
                  if (cell) {
                    var diagramData = cell.value.userObject;
                    if (Hmi.isControllable(diagramData.type)) {
                      var state = this.graph.view.getState(cell, false);
                      if (state) {
                        var node = state.shape.node;                                            
                        var image = node.getElementsByTagName("image")[0];                        
                        var ref = image.getAttribute('href');
                        if (!ref) {
                          ref = image.getAttribute('xlink:href');
                          image.removeAttribute('xlink:href');
                        }

                        const val = Helpers.convertPos(update.topic.value?.Double);
                        cell.value.userObject.tag = val;
                        cell.value.userObject.lastUpdate = ts;

                        image.setAttribute("href", baseToolbarImagePath + objType + '-' + val + '.svg');                                      
                      }
                    }
                  }
                }
              }
              catch (e) {
                console.error(e);
              }
            }
            else {
              if (Symbol.statusIndicator === objType) {
                const cellId = domElement[i].getAttribute('cell-id');

                var cell = this.graph.getModel().getCell(cellId);
                if (cell) {
                  var diagramData = cell.value.userObject;
                  var color = 'gray';
                  if (diagramData && diagramData.statusDefinition) {
                    for(var j = 0; j < diagramData.statusDefinition.length; ++j) {                       
                      if (diagramData.statusDefinition[j].value == update.topic.value.Double) {
                        color = diagramData.statusDefinition[j].color;
                      }
                      else if (diagramData.statusDefinition[j].value == (update.topic.value.Bool + "")) {
                        color = diagramData.statusDefinition[j].color;
                      }
                    }
                  }
                  // Get img element
                  var img = domElement[i].firstElementChild;
                  img.setAttribute('src', baseImagePath + color + '.svg');
                  cell.value.userObject.lastUpdate = ts;
                }
              }
              else {
                // This is measurement box
                domElement[i].textContent = this.setDataFieldValue(domElement[i], update.topic?.value);
                const cellId = domElement[i].getAttribute('cell-id');
                var cell = this.graph.getModel().getCell(cellId);
                if (cell) {
                  cell.value.userObject.lastUpdate = ts;
                }
              }
            }
          }
        }
      }
    }        
  }  

  sendWsData(data: any) {    
    this.wsService.sendWsData(data);
  }

  sendTapChangerCommand(userObject: DiagramData, action: string, path: string, value?: number) {
    if (!path) {
      this.snack.open('Unable to send command.  No data is mapped for this control.', 'OK', { duration: 2000 });
    }
    else {
      const t : Topic = {
        name: path,
        mrid: userObject.mRID,
        action: action,   
        args: value,     
      };

      const data: UpdateData = {
        topic: t
      };

      this.diagramService.updateData( data)
        .subscribe(data => {                              
          // success
        }, error => {
          console.error(error);
          this.snack.open(error, 'OK', { duration: 4000 });
      });
    }
  }

  sendCommand(userObject: DiagramData, action: string, value?: any) {
    console.log("Sending command with action " + action + " with value: " + value);         
    
    if (action == CommandAction.VERB)
    {
      let command: Command = {
        name: value
      };
      this.diagramService.executeCommand(command)
        .subscribe(data => {                              
          this.snack.open("Command executed successfully.", 'OK', { duration: 4000 });
        }, error => {
          console.error(error);
          this.snack.open(error, 'OK', { duration: 4000 });
      });
    }
    else if (userObject?.controlData?.length > 0) {
      const control = userObject?.controlData[0];

      if (!control.path) {
        this.snack.open('Unable to send command.  No sepcified data connection.', 'OK', { duration: 2000 });
      }
      else if (!userObject.mRID) {
        this.snack.open('Unable to send command.  No sepcified mRID.', 'OK', { duration: 2000 });
      }

      const t : Topic = {
        name: control?.path,
        mrid: userObject.mRID,
        action: action,   
        args: value,     
      };

      const data: UpdateData = {
        topic: t
      };

      this.diagramService.updateData( data)
        .subscribe(data => {                              
          // success
        }, error => {
          console.error(error);
          this.snack.open(error, 'OK', { duration: 4000 });
        });
    }
    else {
      this.snack.open('Unable to send command.  No data is mapped for this control.', 'OK', { duration: 2000 });
    }
  }

  // zoom graph
  zoomGraph(scale: number) {
    this.graph.zoomTo(scale, false);
  }

  loadGraphFromServer(id: string) {
    this.diagramService.get(id).subscribe(
      data => {
        this.currentDiagram = data;

        try {
          if (this.currentDiagram.data && this.currentDiagram.data != "") {
            var xml = mxUtils.parseXml(this.currentDiagram.data);
            var dec = new mxCodec(xml);
            dec.decode(xml.documentElement, this.graph.getModel());
          }
        }
        catch (e) {
          console.error(e);
        }
      },
      error => {
        console.error(error);
        this.snack.open(error, 'OK', { duration: 4000 });
      }
    ); 
  }

  register() {            
    var request = {
      session_id: this.sessionId,
      topics: []
    };

    // Build RegisterRequest object
    var parent = this.graph.getDefaultParent();
    var vertices = this.graph.getChildVertices(parent);
    for (let cell of vertices) {
      const cellValue = this.graph.model.getValue(cell);
      if (cellValue && cellValue.userObject) {
        for (let displayData of cellValue.userObject.displayData) {
          if (displayData.path && displayData.path !== "" && cellValue.userObject.mRID && cellValue.userObject.mRID !== "")
          {
            var topic = {
              name: displayData.path,
              mrid: cellValue.userObject.mRID
            };
            request.topics.push(topic);
          }
        }              
      }
    }    
    this.sendWsData(request);
  }

  setDataFieldValue(element: Element, value: any): string {
    if (typeof value.Double !== 'undefined') {
      if (element.className.match(/\bfield-item-value-state\b/)) {
        if (value.Double === 0.0 ) {
          element.classList.add('red-color-value');
          element.classList.remove('green-color-value');
          return 'off';
        } else {
          element.classList.add('green-color-value');
          element.classList.remove('red-color-value');
          return 'on';
        }
      } else if (element.className.match(/\bfield-item-value-status\b/)) {
        if (value.Double === 1.0) {
          element.classList.add('red-color-value');
          element.classList.remove('green-color-value');
          return 'closed';
        } else {
          element.classList.add('green-color-value');
          element.classList.remove('red-color-value');
          return 'open';
        }
      } else if (element.className.match(/\bfield-item-value-mode\b/)) {
        if (value.Double === 2000.0) {        
          return 'VSI_PQ';
        } else if (value.Double === 2001.0) {        
          return 'VSI_VF';
        } else if (value.Double === 2002.0) {        
          return 'VSI_ISO';
        }
      }  
      return parseFloat(value.Double).toFixed(2).toString();
    }
    else if (value.String) {
      return value.String
    }
    else if (value.Bool) {
      if (value.Bool === true) {
        element.classList.add('green-color-value');
        element.classList.remove('red-color-value');
        return 'on';
      }
      else {
        element.classList.add('red-color-value');
        element.classList.remove('green-color-value');
        return 'off';
      }
    }
    else {
      return '';
    }
  }

  ngOnDestroy() {
    this.destroy$.next();
    this.destroy$.complete();
  }
}
