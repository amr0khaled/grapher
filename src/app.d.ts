/// <reference types="@sveltejs/vite-plugin-svelte" />

declare module '*.css';
declare module 'katex/dist/katex.min.css';

declare namespace App {
  interface Error {
    message: string;
    code?: string;
  }
  interface Locals {}
  interface PageData {}
  interface PageState {}
  interface Platform {}
}

export {};
