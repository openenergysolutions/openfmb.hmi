import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';

@Component({
  selector: 'app-properties-select-section',
  templateUrl: './properties-select-section.component.html',
  styleUrls: ['./properties-select-section.component.scss']
})
export class PropertiesSelectSectionComponent implements OnInit {
  @Input() fields = [];
  @Input() selectedFields = [];
  @Output() emitter = new EventEmitter();

  constructor() { }

  ngOnInit() {
  }


  // select field
  selectField(item) {
    const index = this.selectedFields.findIndex( element => element.value === item.value);
    if (index === -1) {
      const selectedElem = {...item};
      delete selectedElem.selected;
      this.selectedFields.push(selectedElem);
    } else {
      this.selectedFields.splice(index, 1)
    }
    this.emitter.emit(this.selectedFields);
  }

  // select all fields
  selectAllFields() {
    this.selectedFields = [];
    this.fields.forEach( field => {
      field.selected = true;
      const selectedElem = {...field};
      delete selectedElem.selected;
      this.selectedFields.push(selectedElem);
    });
    this.emitter.emit(this.selectedFields);
  }
}
