export namespace WasiIoStreams {
  export { OutputStream };
  export { InputStream };
}
export type StreamError = StreamErrorLastOperationFailed | StreamErrorClosed;
export interface StreamErrorLastOperationFailed {
  tag: 'last-operation-failed',
  val: Error,
}
export interface StreamErrorClosed {
  tag: 'closed',
}

export class InputStream {
}

export class OutputStream {
  blockingWriteAndFlush(contents: Uint8Array): void;
}
