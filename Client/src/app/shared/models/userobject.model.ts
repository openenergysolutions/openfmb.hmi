// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

export interface DiagramData {
    diagramId?: string,
    label?: string,
    description?: string,
    name?: string,
    mRID?: string,
    deviceTypeMapping?: string, // used anywhere?
    left?: number,
    top?: number,
    type?: string, // type of graph item
    func?: string, // button function 
    verb?: string, // command verb
    linkData?: LinkData, // store link data when func == ButtonFunction.link
    textAlign?: string,
    fontSize?: number,
    fontStyle?:string,
    foreColor?: string,
    backgroundColor?: string,
    containerWidth?: number,
    containerHeight?: number,
    changeStyleAllowed?: boolean,    
    tag?: any,
    visible?: boolean,
    lastUpdate?: any,
    statusDefinition?: StatusDefinition[],
    arrowDirection?: ArrowDirection,
    weatherDefinition?: WeatherStatusDefinition[],
    displayData: any[],
    controlData: any[],
    visibilityData: any[]    
};

export interface LinkData {
    url?: string,
    diagramId?: string,
    target?: string
}

export interface StatusDefinition {
    value?: any,    
    color?: string,
    text?: string
}

export interface ArrowDirection {
    positive?: string,
    negative?: string,
    neutral?: string,
    positiveColor?: string,
    negativeColor?: string,
    neutralColor?: string
}

export interface WeatherStatusDefinition {
    from?: number,
    to?: number,
    text?: string
}