export interface Topic {
  name: string,
  mrid: string,
  value?: any,
  action?: string,
  args?: number,
}

export interface UpdateData {
  topic?: Topic
}