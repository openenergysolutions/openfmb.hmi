export interface DiagramData {
    diagramId?: string,
    label?: string,
    name?: string,
    mRID?: string,
    deviceTypeMapping?: string,
    left?: number,
    top?: number,
    type?: string,
    fontSize?: number,
    foreColor?: string,
    backgroundColor?: string,
    containerWidth?: number,
    changeStyleAllowed?: boolean,
    linkData?: LinkData,
    tag?: any,
    statusDefinition?: StatusDefinition[],
    displayData: any[],
    controlData: any[]    
};

export interface LinkData {
    url?: string,
    diagramId?: string,
    target?: string
}

export interface StatusDefinition {
    value?: any,    
    color?: string
}