export interface ProgressEvent {
  phase: string;
  current: number;
  total: number;
  item: string | null;
}

export interface ConvertResult {
  zip_path: string;
  file_count: number;
  image_count: number;
}

export interface Format {
  id: string;
  name: string;
  available: boolean;
  desc: string;
  highlight: string;
}

export const formats: Format[] = [
  { id: "yolo", name: "YOLO26", available: true, desc: "TXT annotations and YAML config", highlight: "YOLO26" },
  { id: "yolo", name: "YOLOv8", available: true, desc: "TXT annotations and YAML config", highlight: "YOLOv8" },
  { id: "yolo", name: "YOLOv5", available: true, desc: "TXT annotations and YAML config", highlight: "YOLOv5" },
  { id: "coco", name: "COCO JSON", available: true, desc: "COCO JSON annotations", highlight: "EfficientDet Pytorch and Detectron 2" },
];
