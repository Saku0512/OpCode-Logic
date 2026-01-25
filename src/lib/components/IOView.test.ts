import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte/svelte5';
import IOView from './IOView.svelte';

describe('IOView', () => {
  it('renders all sections', () => {
    render(IOView, { input: [], output: [], expected: [0] });
    
    expect(screen.getByText('Input Queue')).toBeInTheDocument();
    expect(screen.getByText('Output Queue')).toBeInTheDocument();
    expect(screen.getByText('Expected Output')).toBeInTheDocument();
  });

  it('displays input values', () => {
    const input = [1, 2, 3];
    render(IOView, { input, output: [], expected: [] });
    
    input.forEach(val => {
      expect(screen.getByText(val.toString())).toBeInTheDocument();
    });
  });

  it('displays output values', () => {
    const output = [10, 20, 30];
    render(IOView, { input: [], output, expected: [] });
    
    output.forEach(val => {
      expect(screen.getByText(val.toString())).toBeInTheDocument();
    });
  });

  it('displays expected values', () => {
    const expected = [100, 200];
    render(IOView, { input: [], output: [], expected });
    
    expected.forEach(val => {
      expect(screen.getByText(val.toString())).toBeInTheDocument();
    });
  });

  it('handles empty arrays', () => {
    render(IOView, { input: [], output: [], expected: [] });
    
    expect(screen.getByText('Input Queue')).toBeInTheDocument();
    expect(screen.getByText('Output Queue')).toBeInTheDocument();
    expect(screen.queryByText('Expected Output')).not.toBeInTheDocument();
  });

  it('displays negative numbers correctly', () => {
    const input = [-5, -10];
    render(IOView, { input, output: [], expected: [] });
    
    expect(screen.getByText('-5')).toBeInTheDocument();
    expect(screen.getByText('-10')).toBeInTheDocument();
  });
});
