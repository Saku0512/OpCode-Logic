import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte/svelte5';
import Editor from './Editor.svelte';

describe('Editor', () => {
  it('renders textarea with placeholder', () => {
    render(Editor, { code: '' });
    const textarea = screen.getByPlaceholderText('Enter assembly code here...');
    expect(textarea).toBeInTheDocument();
  });

  it('displays initial code value', () => {
    const initialCode = 'mov rax, 10';
    render(Editor, { code: initialCode });
    const textarea = screen.getByDisplayValue(initialCode);
    expect(textarea).toBeInTheDocument();
  });

  it('has correct styling classes', () => {
    const { container } = render(Editor, { code: '' });
    const editorContainer = container.querySelector('.editor-container');
    const editor = container.querySelector('.editor');
    
    expect(editorContainer).toBeInTheDocument();
    expect(editor).toBeInTheDocument();
  });
});
