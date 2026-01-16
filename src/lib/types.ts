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
