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
    displayData: []
};

export interface LinkData {
    url?: string,
    diagramId?: string,
    target?: string
}