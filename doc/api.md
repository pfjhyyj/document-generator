# API Definition

## Generation Request

```ts
interface GenerationRequest {
  templateFile: string
  dataItem: DataItem
}

interface DataItem {
 [key: string]: number | number[] | string | string[] | DataItem | DataItem[]
}

interface ResultItem {
  name: string
  type: string
  keyName: string
  displayMode: number
  // format:
  // properties: 
}

interface ImageFormat {
  width: number
  height: number
  mode: number
}

enum ImageRenderMode {

}
```
