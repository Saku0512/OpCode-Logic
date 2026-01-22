import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte/svelte5';
import RegisterView from './RegisterView.svelte';

describe('RegisterView', () => {
  it('renders all registers', () => {
    render(RegisterView, { registers: {} });
    
    const registerNames = ['RAX', 'RBX', 'RCX', 'RDX', 'RSI', 'RDI', 'RSP', 'RBP', 
                          'R8', 'R9', 'R10', 'R11', 'R12', 'R13', 'R14', 'R15'];
    
    registerNames.forEach(name => {
      expect(screen.getByText(name)).toBeInTheDocument();
    });
  });

  it('displays register values correctly', () => {
    const registers = {
      RAX: 100,
      RBX: 200,
      RCX: 300,
    };
    
    render(RegisterView, { registers });
    
    expect(screen.getByText('100')).toBeInTheDocument();
    expect(screen.getByText('200')).toBeInTheDocument();
    expect(screen.getByText('300')).toBeInTheDocument();
  });

  it('displays 0 for undefined registers', () => {
    render(RegisterView, { registers: {} });
    
    // Should display 0 for all registers when not provided
    const zeros = screen.getAllByText('0');
    expect(zeros.length).toBeGreaterThan(0);
  });

  it('has correct heading', () => {
    render(RegisterView, { registers: {} });
    expect(screen.getByText('Registers (x86_64)')).toBeInTheDocument();
  });
});
