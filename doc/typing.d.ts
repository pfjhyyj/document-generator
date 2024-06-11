interface GenerationRequest {
  templateFile: string
  dataItem: DataItem
  resultItem: ResultItem[]
}

interface DataItem {
 [key: string]: number | number[] | string | string[] | DataItem | DataItem[]
}

interface ResultItem {
  name: string
  type: DataItemType
  keyName: string
  renderMode: RenderMode
  renderFormat: RenderFormat
  properties: ResultItem[]
}

export enum DataItemType {
  STRING = 1,
  NUMBER = 2,
  BOOLEAN = 3,
  OBJECT = 4,
  ARRAY = 5
}

export enum RenderMode {
  STRING = 1,
  NUMBER = 2,
  IMAGE = 3,
  TABLE = 4,
  CHECKBOX = 5,

  EMBEDDED = 98,
  TEMPLATE = 99
}

interface ImageFormat {
  width: number
  height: number
  mode: ImageRenderMode
}

export enum ImageRenderMode {
  AUTO_SIZE = 1,
  FIX_SIZE = 2
}


interface NumberFormat {
  precision: number
}

interface TemplateFormat {
  template: string
}

interface TableFormat {
  repeatHeader: boolean
  headerRowLength: number
  mergeColumn: boolean
  mergeColumnSize: number
}

type RenderFormat = ImageFormat | NumberFormat | TemplateFormat | TableFormat