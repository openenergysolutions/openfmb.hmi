export interface Topic {
  name: string,
  mrid: string,
  value?: any
}

export interface UpdateData {
  topic?: Topic
}