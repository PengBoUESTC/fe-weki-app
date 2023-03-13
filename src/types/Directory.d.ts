export interface Directory {
  name: String;
  path: string;
  parent: string | null;
  children?: Directory[];
}
