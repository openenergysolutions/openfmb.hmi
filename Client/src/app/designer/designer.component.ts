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
import { PropertiesDialogComponent } from './dialogs/properties-dialog/properties-dialog.component';
import { NgxSpinnerService } from 'ngx-spinner';
import toolbarItemsData from '../../assets/json/toolbar.json';
import * as fromRoot from '../store/reducers/index';
import { WebSocketService } from '../core/services/web-socket.service';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { v4 as uuidv4 } from 'uuid';
import { ActivatedRoute } from '@angular/router'
import { DiagramsService } from '../shared/services/diagrams.service';
import { DiagramData } from '../shared/models/userobject.model'
import { MatSnackBar } from '@angular/material/snack-bar';
import { Router } from '@angular/router';
import { Diagram } from '../shared/models/diagram.model';
import { Helpers, Hmi, Symbol } from '../shared/hmi.constants'
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
  mxKeyHandler,
  mxUndoManager,
  mxClient
} = mxgraphFactory({
  mxLoadResources: false,
  mxLoadStylesheets: false,
});

@Component({
  selector: 'app-designer',
  templateUrl: './designer.component.html',
  styleUrls: ['./designer.component.scss']
})
export class DesignerComponent implements OnInit, AfterViewInit, OnDestroy {
  @ViewChild('graphContainer', { static: false }) graphContainer: ElementRef;
  @ViewChild('toolbarContainer', { static: false }) toolbarContainer: ElementRef;
  DESIGNER_CONST = DesignerConstant;
  graph: mxgraph.mxGraph;
  toolbar: mxgraph.mxToolbar;
  mode = this.DESIGNER_CONST.SELECT_MODE;
  canvas: any;
  toolbarItems = toolbarItemsData;
  sessionId = '';  
  defaultLabelStyle = 'text;html=1;strokeColor=none;fontColor=#000000;verticalAlign=middle;whiteSpace=wrap;rounded=0;fillColor=none;';
  defaultButtonStyle = 'html=1;strokeColor=none;verticalAlign=middle;whiteSpace=wrap;rounded=1;';
  defaultSetpointButtonStyle = 'html=1;strokeColor=none;align=center;verticalAlign=middle;whiteSpace=wrap;rounded=1;fillColor=#a6a6af;fontColor=#0e0e0f;';
  defaultStatusIndicatorStyle = 'html=1;strokeColor=none;align=left;verticalAlign=middle;rounded=1;fillColor=none;fontColor=#000000;';
  defaultRectangleGroupStyle = 'rounded=0;whiteSpace=wrap;html=1;dashed=1;shadow=0;sketch=0;glass=0;fillColor=none;strokeWidth=2;strokeColor=#606060';
  gridData = {
    scale: 0,
    gridSize: 0,
    translationPoint: new mxPoint(),
    width: 0,
    height: 0
  };
  diagramId: string = null;  
  currentDiagram: Diagram;
  undoManager: mxgraph.mxUndoManager;

  private destroy$ = new Subject();

  private interval;

  constructor(
    private renderer: Renderer2,
    private store: Store<fromRoot.State>,
    public dialog: MatDialog,
    private spinner: NgxSpinnerService,
    private wsService: WebSocketService,    
    private router : ActivatedRoute,
    private diagramService: DiagramsService,
    private snack: MatSnackBar,
    private naviator: Router,
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

    // initialize toolbar.
    this.toolbarInit();

    // load default styles
    this.defaultGraphStyles();

    // change mode according to selection.
    this.store.select(state => state.designer.mode).pipe(takeUntil(this.destroy$)).subscribe(data => {
      this.mode = data;
      this.changeMode(this.mode);
    });

    // change connection color according to selection.
    this.store.select(state => state.designer.connectColor).pipe(takeUntil(this.destroy$)).subscribe(data => {
      if (data) {
        const edgeStyle = this.graph.getStylesheet().getDefaultEdgeStyle();
        edgeStyle[mxConstants.STYLE_STROKECOLOR] = data;
      }
    });    

    this.sessionId = uuidv4();

    // load graph
    if (this.diagramId) { 
      this.loadGraphFromServer(this.diagramId);       
    }
    else {
      this.naviator.navigateByUrl("**");
    }
  }

  // init graph.
  private graphInit() {
    mxEvent.disableContextMenu(this.graphContainer.nativeElement);
    mxConstraintHandler.prototype.pointImage = new mxImage('../../assets/images/dot.gif', 10, 10);        
    this.graph = new mxGraph(this.graphContainer.nativeElement);
    this.graph.setPortsEnabled(false);
    this.graph.graphHandler.scaleGrid = true;
    this.graph.gridSize = 10;
    this.graph.multigraph = false;
    this.graph.setAllowDanglingEdges(false);
    this.graph.isHtmlLabel = (cell) => {
      return !this.graph.isSwimlane(cell);
    };    
     
    // undo manager
    this.undoManager = new mxUndoManager(20);
    
    this.graph.getModel().addListener(mxEvent.UNDO, (sender, evt) => {
      this.undoManager?.undoableEditHappened(evt.getProperty('edit'));
    });
    this.graph.getView().addListener(mxEvent.UNDO, (sender, evt) => {
      this.undoManager?.undoableEditHappened(evt.getProperty('edit'));
    });

    // Open popup in double click.
    this.graph.dblClick = (evt: MouseEvent, cell: mxgraph.mxCell) => {
      if (
        this.graph.isEnabled() &&
        !mxEvent.isConsumed(evt) &&
        cell != null &&
        this.graph.isCellEditable(cell)
      ) {
        
        const isMeasureBox = this.isMeasureBox(cell);
        
        if (isMeasureBox || this.graph.isHtmlLabel(cell) || (evt.target as HTMLElement).tagName === 'image' || (evt.target as HTMLElement).tagName === 'path') {
          // Assign MRID, label
          const centerX = window.innerWidth / 2; 
          const centerY = window.innerHeight / 2;          
          const x = evt.offsetX;
          const y = evt.offsetY;

          this.openDialog(x < centerX ? x + cell.getGeometry().width + 250 : x, y - 100 > centerY ? centerY : y, cell);                   
        }
      } else {
        this.graph.view.setTranslate(0, 0);
      }
      mxEvent.consume(evt);
    };

    // close popup on empty graph area click.
    this.graph.addListener(mxEvent.CLICK, (evt: Event) => {
      this.dialog.closeAll();
      mxEvent.consume(evt);
    });

    this.graph.getModel().addListener(mxEvent.CHANGE, (evt: Event) => {
      //console.log("Graph model has changed!");  
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
      if (dx && cells[0].value.userObject?.left) {
        dx = dx + cells[0].value.userObject?.left
      }

      if (dy && cells[0].value.userObject?.top) {
        dy = dy + cells[0].value.userObject?.top
      }

      cellsMoved.apply(this.graph, [cells, dx, dy, isconnect, constrain, extend]);
    };

    const insertEdge = this.graph.insertEdge;
    this.graph.insertEdge = (...params) => {
      const color = this.graph.getStylesheet().getDefaultEdgeStyle().strokeColor;
      params[1] = this.idGenerator();
      params[5] = `strokeColor=${color};`;
      return insertEdge.apply(this.graph, params)
    };

    // sets label and data fields to grid item
    this.graph.convertValueToString = (cell) => {            
      const cellValue = this.graph.model.getValue(cell);
      if (cellValue && cellValue.userObject) {
        const userObject = cellValue.userObject;
        const displayData = cellValue.userObject.displayData;
        
        // Returns a DOM for the label
        const wrapper = this.renderer.createElement('div');
        this.renderer.addClass(wrapper, 'component-label');        

        if (userObject.type === Symbol.measureBox) {          
          if (displayData) {
            const data = this.renderer.createElement('div');
            this.renderer.addClass(data, 'display-data-container');   
            
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

            if (style != '') {
              this.renderer.setAttribute(data, "style", style);
            }
            
            const titleContainer = this.renderer.createElement('div');
            const titleText = this.renderer.createText(userObject.label ? userObject.label : userObject.deviceTypeMapping ? userObject.deviceTypeMapping : "Measure");
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
              }
              const value = this.renderer.createText(text);
              this.renderer.addClass(fieldItem, 'field-item');
              this.renderer.addClass(fieldItemLabel, 'field-item-label');
              this.renderer.addClass(fieldItemValue, 'field-item-value');
              this.renderer.appendChild(fieldItemLabel, label);
              this.renderer.appendChild(fieldItemValue, value);
              this.renderer.appendChild(fieldItemText, fieldItemValue);
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
          const div = this.renderer.createElement('div');

          const span = this.renderer.createElement('span');
          this.renderer.addClass(span, 'button-text');
          
          if (style != '') {
            this.renderer.setAttribute(span, "style", style);
          }                      

          const labelText = this.renderer.createText(userObject.label);
          this.renderer.appendChild(span, labelText);
          
          this.renderer.appendChild(div, span);
          
          return div;
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
        //const elt = cell.value.obj.cloneNode(true);
        //elt.setAttribute('label', newValue);
        //newValue = elt;
        userObject.label = newValue;
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

    this.drawGrid();

    // enable graph pan.
    this.graph.createPanningManager = () => {
      const pm = new mxPanningManager(this.graph);
      pm.border = 30;
      return pm;
    };

    // enable right click popup.
    this.graph.popupMenuHandler.factoryMethod = (menu: mxgraph.mxPopupMenu, cell: mxgraph.mxCell, evt: Event) => {
      if (this.mode === this.DESIGNER_CONST.SELECT_MODE) {
        if (cell && (cell.vertex || cell.edge)) {
          menu.addItem('Remove', null, () => {                        
            this.graph.removeCells();
          });
          menu.addItem('Send to Front', null, () => {                        
            this.graph.orderCells(false, [cell]);
          });
          menu.addItem('Send to Back', null, () => {                        
            this.graph.orderCells(true, [cell]);
          });
        }
      }
    };

    const keyHandler = new mxKeyHandler(this.graph);

    keyHandler.getFunction = function(evt)
    {
      if (evt != null)
      {
        return (mxEvent.isControlDown(evt) || (mxClient.IS_MAC && evt.metaKey)) ? this.controlKeys[evt.keyCode] : this.normalKeys[evt.keyCode];
      }
      return null;
    };

    // Handle delete key
    keyHandler.bindKey(46, (evt: Event) => {      
      if (this.graph.isEnabled()) {                
        this.graph.removeCells();
      }
      else {
        console.log("Key pressed by graph is NOT enabled???");
      }
    });

    // Handle undo (ctrl-z)
    keyHandler.bindControlKey(90, (evt: Event) => {            
      if (this.graph.isEnabled()) {
        this.undoManager.undo();
      }      
    });

    // Handle redo (ctrl-y)
    keyHandler.bindControlKey(89, (evt: Event) => {           
      if (this.graph.isEnabled()) {
        this.undoManager.redo();
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

  private openMeasureBoxDialog(cell: mxgraph.mxCell, x: number, y: number) {
    // Assign measurement box
    const centerX = window.innerWidth / 2;    
    this.openDialog(x < centerX ? x + cell.getGeometry().width + 250 : x, y, cell);
  }

  private isMeasureBox(cell: mxgraph.mxCell) {
    if (cell.value != null && cell.value.userObject != null) {
      if (cell.value.userObject.type != null && cell.value.userObject.type == "measure-box") {
        return true;
      }
    }
    return false; 
  }

  // init toolbar.
  private toolbarInit() {
    this.toolbar = new mxToolbar(this.toolbarContainer.nativeElement);
    this.toolbarItems.forEach(item => {
      this.addSidebarIcon(this.graph, this.toolbarContainer, item);
    });
  }

  // add toolbar icons.
  private addSidebarIcon(graphItem: mxgraph.mxGraph, sidebar: ElementRef, data: any) {    
    const imgDropFunction = (graph: mxgraph.mxGraph, evt: Event, cell: mxgraph.mxCell, x: number, y: number) => {      
      if (this.mode === this.DESIGNER_CONST.SELECT_MODE) {        
        const parent = graph.getDefaultParent();
        const model = graph.getModel();
        const currentValue = model.getValue(cell);
        let v1: mxgraph.mxCell;
        const userObject : DiagramData = {
          label: '',
          name: data.title,
          mRID: '',
          deviceTypeMapping: '',
          left: data.left,
          top: data.top,
          type: data.type,                    
          foreColor: '',
          backgroundColor: '',
          displayData: [],
          controlData: []
        };
        model.beginUpdate();
        try {

          if (data.shape == Symbol.image) {
            const ports = [];
            const pointX = (data.left && Math.round(x) === x) ? x + data.left : x;
            const pointY = (data.top && Math.round(y) === y) ? y + data.top : y;
            
            v1 = graph.insertVertex(parent, this.idGenerator(), data.label === false ? null : userObject, pointX, pointY,
              data.width ? data.width : 40, data.height ? data.height : 40, `shape=${data.shape};${data.image ? 
              'image=' + data.image + ';' : ''}${data.rotation ? 'rotation='+ data.rotation +';' : ''}`);              
            data.points.forEach(point => {
              ports.push({ x: point[0], y: point[1], perimeter: data.pointPerimeter === false ? data.pointPerimeter : true });
            });
            model.setValue(v1, { ...currentValue, ports, userObject });
            Object.defineProperty(v1, 'getPorts', {
              value: () => {
                return ports;
              }
            });

            if (this.isMeasureBox(v1)) {            
              this.openMeasureBoxDialog(v1, x, y);
            }
          }
          else if (data.shape == Symbol.text) {
            v1 = graph.insertVertex(parent, this.idGenerator(), 'Label', x, y, 80, 40, this.defaultLabelStyle);
            userObject.label = 'Text';
            model.setValue(v1, { ...currentValue, userObject });            
          }
          else if (data.shape == Symbol.button) {
            v1 = graph.insertVertex(parent, this.idGenerator(), 'Button', x, y, 80, 40, this.defaultButtonStyle + 'align=center;' + 'fillColor=' + Helpers.buttonBackColor() + ';fontColor=' + Helpers.buttonForeColor() + ';');
            userObject.label = 'Button';
            model.setValue(v1, { ...currentValue, userObject });            
          } 
          else if (data.shape == Symbol.setPointButton) {
            v1 = graph.insertVertex(parent, this.idGenerator(), 'Button', x, y, 80, 40, this.defaultSetpointButtonStyle);
            userObject.label = 'Set Point';
            model.setValue(v1, { ...currentValue, userObject });            
          } 
          else if (data.shape == Symbol.statusIndicator) {
            v1 = graph.insertVertex(parent, this.idGenerator(), 'Button', x, y, 80, 40, this.defaultStatusIndicatorStyle);
            userObject.label = 'Status';
            model.setValue(v1, { ...currentValue, userObject });
          }
          else if (data.shape == Symbol.rectangle) {
            v1 = graph.insertVertex(parent, this.idGenerator(), '', x, y, 80, 40, this.defaultRectangleGroupStyle);
            userObject.label = 'Group';
            model.setValue(v1, { ...currentValue, userObject });
          }
        } finally {
          model.endUpdate();
        }
        graphItem.setSelectionCell(v1);
      }
    };

    // Creates the image which is used as the sidebar icon (drag source)
    const wrapper = this.renderer.createElement('div');
    const img = this.renderer.createElement('img');
    const title = this.renderer.createElement('div');
    const titleText = this.renderer.createText(data.title);
    this.renderer.setAttribute(img, 'src', data.image.split('.svg')[0] + '-copy.svg');    
    this.renderer.addClass(wrapper, 'toolbar-wrapper');
    this.renderer.addClass(img, 'toolbar-image');
    this.renderer.addClass(title, 'toolbar-title');
    this.renderer.appendChild(title, titleText);
    this.renderer.appendChild(wrapper, img);
    this.renderer.appendChild(wrapper, title);
    this.renderer.appendChild(sidebar.nativeElement, wrapper);

    const dragSource = this.renderer.createElement('div');
    this.renderer.setStyle(dragSource, 'width', (data.width ? data.width : 40) + 'px');
    this.renderer.setStyle(dragSource, 'height', (data.height ? data.height : 40) + 'px');
    this.renderer.setStyle(dragSource, 'border', '1px solid black');
    this.renderer.setStyle(dragSource, 'border-style', 'dashed');
    this.renderer.setStyle(dragSource, 'margin-left', (data.left ? data.left : 0) + 'px');
    this.renderer.setStyle(dragSource, 'margin-top', (data.top ? data.top : 0) + 'px');
    if(data.rotation) {
      this.renderer.setStyle(dragSource, 'transform', `rotate(${data.rotation}deg)`);
    }

    const ds = mxUtils.makeDraggable(wrapper, graphItem, imgDropFunction, dragSource, 0, 0, true, true, true);
    ds.drag = '';
    ds.setGuidesEnabled(true);
  }

  // paint grid canvas in graph.
  private repaintGrid(canvas: ElementRef, graph: mxgraph.mxGraph, s: number, gs: number, tr: mxgraph.mxPoint, w: number, h: number) {
    const ctx = canvas.nativeElement.getContext('2d');
    if (ctx !== null) {
      const bounds = graph.getGraphBounds();
      const width = Math.max(bounds.x + bounds.width, graph.container.clientWidth);
      const height = Math.max(bounds.y + bounds.height, graph.container.clientHeight);
      const sizeChanged = width !== w || height !== h;
      if (graph.view.scale !== s || graph.view.translate.x !== tr.x || graph.view.translate.y !== tr.y ||
        gs !== graph.gridSize || sizeChanged) {
        tr = graph.view.translate.clone();
        s = graph.view.scale;
        gs = graph.gridSize;
        w = width;
        h = height;

        // Clears the background if required
        if (!sizeChanged) {
          ctx.clearRect(0, 0, w, h);
        } else {
          canvas.nativeElement.setAttribute('width', w);
          canvas.nativeElement.setAttribute('height', h);
        }

        const tx = tr.x * s;
        const ty = tr.y * s;

        // Sets the distance of the grid lines in pixels
        const minStepping = graph.gridSize;
        let stepping = minStepping * s;

        if (stepping < minStepping) {
          const count = Math.round(Math.ceil(minStepping / stepping) / 2) * 2;
          stepping = count * stepping;
        }

        const xs = Math.floor((0 - tx) / stepping) * stepping + tx;
        let xe = Math.ceil(w / stepping) * stepping;
        const ys = Math.floor((0 - ty) / stepping) * stepping + ty;
        let ye = Math.ceil(h / stepping) * stepping;

        xe += Math.ceil(stepping);
        ye += Math.ceil(stepping);

        const ixs = Math.round(xs);
        const ixe = Math.round(xe);
        const iys = Math.round(ys);
        const iye = Math.round(ye);

        // Draws the actual grid        
        ctx.strokeStyle = '#d1d1d1';
        ctx.beginPath();        

        for (let x = xs; x <= xe; x += stepping) {          
          x = Math.round((x - tx) / stepping) * stepping + tx;
          const ix = Math.round(x);

          ctx.moveTo(ix + 1, iys + 1);
          ctx.lineTo(ix + 1, iye + 1);           
        }

        for (let y = ys; y <= ye; y += stepping) {          
          y = Math.round((y - ty) / stepping) * stepping + ty;
          const iy = Math.round(y);
          ctx.moveTo(ixs + 0.5, iy + 0.5);
          ctx.lineTo(ixe + 0.5, iy + 0.5);          
        }
        
        ctx.closePath();
        ctx.stroke();
      }
    }
  }

  // change selection mode.
  private changeMode(mode: number) {
    if (mode === this.DESIGNER_CONST.MOVE_MODE) {
      this.dialog.closeAll();
      this.graph.setPanning(true);
      this.graph.panningHandler.useLeftButtonForPanning = true;
      this.graph.setConnectable(false);
      this.graph.getSelectionModel().clear();
      this.graph.setCellsSelectable(false);
      this.graph.setDropEnabled(false);
      this.graph.setCellsMovable(false);
    } else if (mode === this.DESIGNER_CONST.CONNECT_MODE) {
      this.dialog.closeAll();
      this.graph.setPanning(false);
      this.graph.setConnectable(true);
      this.graph.getSelectionModel().clear();
      this.graph.setCellsSelectable(false);
      this.graph.setDropEnabled(false);
      this.graph.setCellsMovable(false);
    } else if (mode === this.DESIGNER_CONST.SELECT_MODE) {
      this.dialog.closeAll();
      this.graph.setPanning(false);
      this.graph.setConnectable(false);
      this.graph.setCellsSelectable(true);
      this.graph.setDropEnabled(true);
      this.graph.setCellsMovable(true);
    }
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

  // draw grid canvas.
  private drawGrid() {
    try {
      this.canvas = this.renderer.createElement('canvas');
      this.canvas.style.position = 'absolute';
      this.canvas.style.top = '0px';
      this.canvas.style.left = '0px';
      this.graph.container.appendChild(this.canvas);

      const mxGraphViewIsContainerEvent = mxGraphView.prototype.isContainerEvent;
      mxGraphView.prototype.isContainerEvent = (evt: Event) => {
        return mxGraphViewIsContainerEvent.apply(this.graph.getView(), [evt]) ||
          mxEvent.getSource(evt) === this.canvas;
      };
      this.repaintGrid(new ElementRef(this.canvas), this.graph, this.gridData.scale, this.gridData.gridSize, this.gridData.translationPoint, this.gridData.width, this.gridData.height);
    } catch {
      mxLog.show();
      mxLog.debug();
      this.graphContainer.nativeElement.style.backgroundImage = 'url(\'../../assets/images/grid.gif\')';
    }

    mxGraphView.prototype.validateBackground = () => {
      this.repaintGrid(new ElementRef(this.canvas), this.graph, this.gridData.scale, this.gridData.gridSize, this.gridData.translationPoint, this.gridData.width, this.gridData.height);
    };

    mxConstants.MIN_HOTSPOT_SIZE = 16;
    mxConstants.DEFAULT_HOTSPOT = 1;
  }

  // open property popup
  openDialog(x: number, y: number, cell: mxgraph.mxCell): void {
    const currentCellData = this.graph.model.getValue(cell).userObject;      
    this.dialog.closeAll();

    if (currentCellData.type === Symbol.button) {
      if (!currentCellData.backgroundColor) {
        // get the background color
        let index = cell.style?.indexOf('fillColor');
        if (index >= 0) {
          let substr = cell.style.substring(index);
          index = substr.indexOf(';');
          substr = substr.substring(0, index);
          substr = substr.replace(';', '').replace(';', '');                 
          let tokens = substr.split("=");
          if (tokens.length > 1) {
            currentCellData.backgroundColor = tokens[1].trim();
          }
        }
      }
    }
    
    const dialogRef = this.dialog.open(PropertiesDialogComponent, {
      width: '375px',      
      data: currentCellData,
      hasBackdrop: false,
      panelClass: 'filter-popup',      
      autoFocus: true,
      closeOnNavigation: true
    });

    dialogRef.afterClosed().pipe(takeUntil(this.destroy$)).subscribe(result => {
      if (result) {
        console.log("Property dialog closed.  Saving data..." + JSON.stringify(result));
        const currentValue = this.graph.model.getValue(cell);
        const userObject = {
          ...currentValue.userObject,
          label: result.label, 
          name: result.name,   
          statusDefinition: result.statusDefinition,      
          displayData: result.displayData,
          controlData: result.controlData,
          mRID: result.mRID,
          fontSize: result.fontSize,
          fontStyle: result.fontStyle,
          textAlign: result.textAlign,
          containerWidth: result.containerWidth,
          containerHeight: result.containerHeight,
          foreColor: result.foreColor,
          backgroundColor: result.backgroundColor,
          linkData: result.linkData,
          deviceTypeMapping: result.deviceTypeMapping,
          func: result.func,
          verb: result.selectedCommand
        };
        
        this.graph.model.setValue(cell, { ...currentValue, userObject });
        if (currentCellData.type === Symbol.label) {
          let style = this.defaultLabelStyle;
          if (userObject.textAlign) {
            style = style + 'align=' + userObject.textAlign + ';'            
          }
          if (userObject.fontStyle) {
            style = style + 'fontStyle=' + userObject.fontStyle + ';';
          }
          this.graph.setCellStyle(style, [cell]);
        }
        else if (currentCellData.type === Symbol.button) {
          let style = this.defaultButtonStyle;
          if (userObject.textAlign) {
            style = style + 'align=' + userObject.textAlign + ';'            
          }
          if (userObject.fontStyle) {
            style = style + 'fontStyle=' + userObject.fontStyle + ';';
          }
          if (userObject.backgroundColor) {
            style = style + 'fillColor=' + userObject.backgroundColor + ';';
          }
          else {
            style = style + 'fillColor=none;';
          }
          if (userObject.fontColor) {
            style = style + 'fontColor=' + userObject.fontColor + ';';
          }
          else {
            style = style + 'fontColor=' + Helpers.buttonForeColor() + ';';
          }
          this.graph.setCellStyle(style, [cell]);
        }


        if (result.navigateToDataConnection === true) {
          this.saveGraphToServer();

          this.naviator.navigateByUrl("/data-connect?id=" + this.diagramId + "&cell=" + cell.id);
        }
      }
    });
  }  

  // save graph to json file and download.
  saveGraph() {    
    var encoder = new mxCodec();
    var node = encoder.encode(this.graph.getModel());
    var xml = mxUtils.getXml(node);
    const blob = new Blob([xml], { type: 'text/xml;charset=utf-8' });
    const url = window.URL;
    const link = url.createObjectURL(blob);
    const downloadLink = this.renderer.createElement('a');
    this.renderer.setStyle(downloadLink, 'display', 'none');
    this.renderer.setAttribute(downloadLink, 'download', 'design.xml');
    this.renderer.setAttribute(downloadLink, 'href', link);
    downloadLink.click();
  }

  saveGraphToServer() {
    var encoder = new mxCodec();
    var node = encoder.encode(this.graph.getModel());
    var xml = mxUtils.getXml(node);   

    this.currentDiagram.data = xml;

    this.diagramService.update(this.currentDiagram).subscribe(
      response => {
        //console.log("Updated diagram:: " + response),
        this.snack.open('Diagram is updated.', 'OK', { duration: 2000 });        
      },
      error => {
        console.error(error);
        this.snack.open(error, 'OK', { duration: 4000 });
      }
    );    
  }
  
  sendWsData(data: any) {    
    this.wsService.sendWsData(data);
  }

  // zoom graph
  zoomGraph(scale: number) {
    this.graph.zoomTo(scale, false);
  }

  exportGraph() {
    alert('TODO');
  }

  // load graph from json file.
  loadGraphFromFile(file: File) {
    if (file) {
      const fileReader = new FileReader();
      fileReader.readAsText(file, 'UTF-8');
      fileReader.onload = () => {
        var xml = mxUtils.parseXml(fileReader.result);
        var dec = new mxCodec(xml);
        dec.decode(xml.documentElement, this.graph.getModel());
      };
      fileReader.onerror = (error) => {
        console.log(error);
      };
    }
    this.spinner.hide();    
  }

  loadGraphFromServer(id: string) {

    this.diagramService.get(id).subscribe(
      data => {
        this.currentDiagram = data;

        try {
          if (this.currentDiagram.data && this.currentDiagram.data != "") {
            var xml = mxUtils.parseXml(this.currentDiagram.data);
            var dec = new mxCodec(xml);
            try {
              this.graph.getModel().beginUpdate();
              dec.decode(xml.documentElement, this.graph.getModel());                            
            }
            finally {
              this.graph.getModel().endUpdate();              
            }
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

  runGraph() {
    this.saveGraphToServer();
    this.naviator.navigateByUrl('/hmi?id=' + this.diagramId);
  }

  // adds default graph wrapper
  private addGraphContainer(content: any) {
    return {
      elements:[{
          type: 'element',
          name:'mxGraphModel',
          elements:[{
            type:'element',
            name:'root',
            elements: content ? [...content] : []
          }]
      }]
    };
  }

  // generate id for grid item
  idGenerator(): string {    
    return uuidv4();
  };    

  @HostListener('window:resize')
  onResize() {
    this.repaintGrid(new ElementRef(this.canvas), this.graph, this.gridData.scale, this.gridData.gridSize, this.gridData.translationPoint, this.gridData.width, this.gridData.height);
  }

  ngOnDestroy() {
    this.destroy$.next();
    this.destroy$.complete();
  }
}
