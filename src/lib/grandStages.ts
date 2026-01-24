export type GrandStageId = string;

export type GrandStage = {
  id: GrandStageId;
  badge: string; // e.g. "GRAND STAGE 01"
  title: string; // e.g. "The Accumulator"
  description: string;
  /**
   * Level ids belonging to this Grand Stage, in display order.
   * Used for stage completion/unlock and for filtering levels on the play screen.
   */
  levelIds: string[];
};

export const grandStages: GrandStage[] = [
  {
    id: "1",
    badge: "GRAND STAGE 01",
    title: "The Accumulator",
    description: "レジスタ/ALU、フラグ、ジャンプ、ループの基礎を総ざらいする。",
    levelIds: [
      "01_Mov&Call",
      "02_Addition",
      "03_Subtraction",
      "04_TheXORTrick",
      "05_Inc&Dec",
      "06_Unconditional",
      "07_ZeroFlag",
      "08_SignFlag",
      "09_Comparison",
      "10_Countdown",
      "11_Accumulate3",
      "12_TheAccumulator",
    ],
  },
  {
    id: "2",
    badge: "GRAND STAGE 02",
    title: "The Stack",
    description: "push/popを軸に、退避・並べ替え・スタック計算の考え方を身につける。",
    // NOTE: いまはバックエンド側のレベル定義が未実装でもOK。将来ここに追加していくだけで拡張できる。
    levelIds: [
      "13_Push&Pop",
      "14_SwapTwo",
      "15_Duplicate",
      "16_Reverse3",
      "17_ReverseUntil0",
      "18_SumFromStack",
      "19_SafePop",
      "20_RPN_AddOnly",
      "21_Sort3",
      "22_Rotate3",
      "23_MinMaxFromStack",
      "24_TheStackMachine",
    ],
  },
];

export function getGrandStage(id: GrandStageId): GrandStage | undefined {
  return grandStages.find((s) => s.id === id);
}

