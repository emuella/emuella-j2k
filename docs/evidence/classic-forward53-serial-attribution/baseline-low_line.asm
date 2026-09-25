Disassembly of section .text:

00000000002021d0 <emuella_j2k_transform::transform_line_forward_first_low_bounded>:
  2021d0:	55                                              	push   %rbp
  2021d1:	41 57                                           	push   %r15
  2021d3:	41 56                                           	push   %r14
  2021d5:	41 55                                           	push   %r13
  2021d7:	41 54                                           	push   %r12
  2021d9:	53                                              	push   %rbx
  2021da:	50                                              	push   %rax
  2021db:	48 89 cd                                        	mov    %rcx,%rbp
  2021de:	48 89 fb                                        	mov    %rdi,%rbx
  2021e1:	48 29 f3                                        	sub    %rsi,%rbx
  2021e4:	48 39 de                                        	cmp    %rbx,%rsi
  2021e7:	0f 85 a0 02 00 00                               	jne    20248d <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x2bd>
  2021ed:	48 83 fe 01                                     	cmp    $0x1,%rsi
  2021f1:	49 89 f2                                        	mov    %rsi,%r10
  2021f4:	49 83 d2 ff                                     	adc    $0xffffffffffffffff,%r10
  2021f8:	48 83 fe 02                                     	cmp    $0x2,%rsi
  2021fc:	0f 82 ff 00 00 00                               	jb     202301 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x131>
  202202:	4c 8d 5d 01                                     	lea    0x1(%rbp),%r11
  202206:	49 d1 eb                                        	shr    $1,%r11
  202209:	48 83 fd 03                                     	cmp    $0x3,%rbp
  20220d:	bb 02 00 00 00                                  	mov    $0x2,%ebx
  202212:	48 0f 43 dd                                     	cmovae %rbp,%rbx
  202216:	48 ff cb                                        	dec    %rbx
  202219:	48 d1 eb                                        	shr    $1,%rbx
  20221c:	4c 39 db                                        	cmp    %r11,%rbx
  20221f:	4c 89 d9                                        	mov    %r11,%rcx
  202222:	48 0f 42 cb                                     	cmovb  %rbx,%rcx
  202226:	48 8d 46 fe                                     	lea    -0x2(%rsi),%rax
  20222a:	48 39 c1                                        	cmp    %rax,%rcx
  20222d:	48 0f 43 c8                                     	cmovae %rax,%rcx
  202231:	45 31 f6                                        	xor    %r14d,%r14d
  202234:	4c 89 c8                                        	mov    %r9,%rax
  202237:	48 29 f0                                        	sub    %rsi,%rax
  20223a:	49 0f 42 c6                                     	cmovb  %r14,%rax
  20223e:	48 39 c1                                        	cmp    %rax,%rcx
  202241:	48 0f 42 c1                                     	cmovb  %rcx,%rax
  202245:	48 83 f8 03                                     	cmp    $0x3,%rax
  202249:	76 62                                           	jbe    2022ad <emuella_j2k_transform::transform_line_forward_first_low_bounded+0xdd>
  20224b:	48 ff c0                                        	inc    %rax
  20224e:	89 c1                                           	mov    %eax,%ecx
  202250:	83 e1 03                                        	and    $0x3,%ecx
  202253:	bf 04 00 00 00                                  	mov    $0x4,%edi
  202258:	48 0f 45 f9                                     	cmovne %rcx,%rdi
  20225c:	49 89 c6                                        	mov    %rax,%r14
  20225f:	49 29 fe                                        	sub    %rdi,%r14
  202262:	49 8d 04 b0                                     	lea    (%r8,%rsi,4),%rax
  202266:	31 c9                                           	xor    %ecx,%ecx
  202268:	0f 1f 84 00 00 00 00 00                         	nopl   0x0(%rax,%rax,1)
  202270:	0f 10 04 ca                                     	movups (%rdx,%rcx,8),%xmm0
  202274:	0f 10 4c ca 04                                  	movups 0x4(%rdx,%rcx,8),%xmm1
  202279:	0f 10 54 ca 10                                  	movups 0x10(%rdx,%rcx,8),%xmm2
  20227e:	0f c6 c2 88                                     	shufps $0x88,%xmm2,%xmm0
  202282:	0f 10 54 ca 14                                  	movups 0x14(%rdx,%rcx,8),%xmm2
  202287:	0f 28 d9                                        	movaps %xmm1,%xmm3
  20228a:	0f c6 da 88                                     	shufps $0x88,%xmm2,%xmm3
  20228e:	0f c6 ca dd                                     	shufps $0xdd,%xmm2,%xmm1
  202292:	66 0f fe c8                                     	paddd  %xmm0,%xmm1
  202296:	66 0f 72 e1 01                                  	psrad  $0x1,%xmm1
  20229b:	66 0f fa d9                                     	psubd  %xmm1,%xmm3
  20229f:	f3 0f 7f 1c 88                                  	movdqu %xmm3,(%rax,%rcx,4)
  2022a4:	48 83 c1 04                                     	add    $0x4,%rcx
  2022a8:	49 39 ce                                        	cmp    %rcx,%r14
  2022ab:	75 c3                                           	jne    202270 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0xa0>
  2022ad:	4d 8d 3c b0                                     	lea    (%r8,%rsi,4),%r15
  2022b1:	4b 8d 3c 36                                     	lea    (%r14,%r14,1),%rdi
  2022b5:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
  2022c0:	4d 39 f3                                        	cmp    %r14,%r11
  2022c3:	0f 84 37 03 00 00                               	je     202600 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x430>
  2022c9:	4c 39 f3                                        	cmp    %r14,%rbx
  2022cc:	0f 84 3e 03 00 00                               	je     202610 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x440>
  2022d2:	4a 8d 04 36                                     	lea    (%rsi,%r14,1),%rax
  2022d6:	4c 39 c8                                        	cmp    %r9,%rax
  2022d9:	0f 83 45 03 00 00                               	jae    202624 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x454>
  2022df:	42 8b 04 f2                                     	mov    (%rdx,%r14,8),%eax
  2022e3:	42 03 44 f2 08                                  	add    0x8(%rdx,%r14,8),%eax
  2022e8:	42 8b 4c f2 04                                  	mov    0x4(%rdx,%r14,8),%ecx
  2022ed:	d1 f8                                           	sar    $1,%eax
  2022ef:	29 c1                                           	sub    %eax,%ecx
  2022f1:	43 89 0c b7                                     	mov    %ecx,(%r15,%r14,4)
  2022f5:	49 ff c6                                        	inc    %r14
  2022f8:	48 83 c7 02                                     	add    $0x2,%rdi
  2022fc:	4d 39 d6                                        	cmp    %r10,%r14
  2022ff:	72 bf                                           	jb     2022c0 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0xf0>
  202301:	48 8d 3c 75 fe ff ff ff                         	lea    -0x2(,%rsi,2),%rdi
  202309:	48 39 ef                                        	cmp    %rbp,%rdi
  20230c:	0f 83 f8 03 00 00                               	jae    20270a <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x53a>
  202312:	4c 8d 14 75 ff ff ff ff                         	lea    -0x1(,%rsi,2),%r10
  20231a:	49 39 ea                                        	cmp    %rbp,%r10
  20231d:	0f 83 f7 03 00 00                               	jae    20271a <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x54a>
  202323:	48 8d 46 ff                                     	lea    -0x1(%rsi),%rax
  202327:	48 01 f0                                        	add    %rsi,%rax
  20232a:	4c 39 c8                                        	cmp    %r9,%rax
  20232d:	0f 83 fa 03 00 00                               	jae    20272d <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x55d>
  202333:	42 8b 0c 92                                     	mov    (%rdx,%r10,4),%ecx
  202337:	8b 3c ba                                        	mov    (%rdx,%rdi,4),%edi
  20233a:	01 ff                                           	add    %edi,%edi
  20233c:	d1 ff                                           	sar    $1,%edi
  20233e:	29 f9                                           	sub    %edi,%ecx
  202340:	41 89 0c 80                                     	mov    %ecx,(%r8,%rax,4)
  202344:	4c 89 c8                                        	mov    %r9,%rax
  202347:	48 29 f0                                        	sub    %rsi,%rax
  20234a:	0f 86 f0 03 00 00                               	jbe    202740 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x570>
  202350:	41 8b 0c b0                                     	mov    (%r8,%rsi,4),%ecx
  202354:	8d 0c 4d 02 00 00 00                            	lea    0x2(,%rcx,2),%ecx
  20235b:	c1 f9 02                                        	sar    $0x2,%ecx
  20235e:	03 0a                                           	add    (%rdx),%ecx
  202360:	41 89 08                                        	mov    %ecx,(%r8)
  202363:	48 83 fe 02                                     	cmp    $0x2,%rsi
  202367:	0f 82 84 02 00 00                               	jb     2025f1 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x421>
  20236d:	4c 8d 55 ff                                     	lea    -0x1(%rbp),%r10
  202371:	49 d1 ea                                        	shr    $1,%r10
  202374:	48 89 f7                                        	mov    %rsi,%rdi
  202377:	48 f7 d7                                        	not    %rdi
  20237a:	4c 01 cf                                        	add    %r9,%rdi
  20237d:	49 39 fa                                        	cmp    %rdi,%r10
  202380:	49 0f 42 fa                                     	cmovb  %r10,%rdi
  202384:	48 8d 4e fe                                     	lea    -0x2(%rsi),%rcx
  202388:	48 39 cf                                        	cmp    %rcx,%rdi
  20238b:	48 0f 43 f9                                     	cmovae %rcx,%rdi
  20238f:	41 bb 01 00 00 00                               	mov    $0x1,%r11d
  202395:	48 83 ff 08                                     	cmp    $0x8,%rdi
  202399:	0f 82 9b 00 00 00                               	jb     20243a <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x26a>
  20239f:	48 8d 0c b5 0b 00 00 00                         	lea    0xb(,%rsi,4),%rcx
  2023a7:	48 83 f9 10                                     	cmp    $0x10,%rcx
  2023ab:	0f 82 89 00 00 00                               	jb     20243a <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x26a>
  2023b1:	48 8d 0c b5 00 00 00 00                         	lea    0x0(,%rsi,4),%rcx
  2023b9:	48 89 cb                                        	mov    %rcx,%rbx
  2023bc:	48 f7 db                                        	neg    %rbx
  2023bf:	48 83 fb 10                                     	cmp    $0x10,%rbx
  2023c3:	72 75                                           	jb     20243a <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x26a>
  2023c5:	48 ff c7                                        	inc    %rdi
  2023c8:	41 89 fb                                        	mov    %edi,%r11d
  2023cb:	41 83 e3 03                                     	and    $0x3,%r11d
  2023cf:	41 be 04 00 00 00                               	mov    $0x4,%r14d
  2023d5:	4d 0f 45 f3                                     	cmovne %r11,%r14
  2023d9:	48 89 fb                                        	mov    %rdi,%rbx
  2023dc:	4c 29 f3                                        	sub    %r14,%rbx
  2023df:	49 f7 de                                        	neg    %r14
  2023e2:	4e 8d 1c 37                                     	lea    (%rdi,%r14,1),%r11
  2023e6:	49 ff c3                                        	inc    %r11
  2023e9:	4c 01 c1                                        	add    %r8,%rcx
  2023ec:	48 83 c1 04                                     	add    $0x4,%rcx
  2023f0:	31 ff                                           	xor    %edi,%edi
  2023f2:	66 0f 6f 05 96 df e0 ff                         	movdqa -0x1f206a(%rip),%xmm0        # 10390 <anon.ee651107ab5319c6bc273e1a29320aaf.78.llvm.14746981713632465754+0x40>
  2023fa:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
  202400:	0f 10 4c fa 08                                  	movups 0x8(%rdx,%rdi,8),%xmm1
  202405:	0f 10 54 fa 18                                  	movups 0x18(%rdx,%rdi,8),%xmm2
  20240a:	0f c6 ca 88                                     	shufps $0x88,%xmm2,%xmm1
  20240e:	f3 0f 6f 54 b9 fc                               	movdqu -0x4(%rcx,%rdi,4),%xmm2
  202414:	f3 0f 6f 1c b9                                  	movdqu (%rcx,%rdi,4),%xmm3
  202419:	66 0f fe da                                     	paddd  %xmm2,%xmm3
  20241d:	66 0f fe d8                                     	paddd  %xmm0,%xmm3
  202421:	66 0f 72 e3 02                                  	psrad  $0x2,%xmm3
  202426:	66 0f fe d9                                     	paddd  %xmm1,%xmm3
  20242a:	f3 41 0f 7f 5c b8 04                            	movdqu %xmm3,0x4(%r8,%rdi,4)
  202431:	48 83 c7 04                                     	add    $0x4,%rdi
  202435:	48 39 fb                                        	cmp    %rdi,%rbx
  202438:	75 c6                                           	jne    202400 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x230>
  20243a:	49 ff c2                                        	inc    %r10
  20243d:	49 8d 1c b0                                     	lea    (%r8,%rsi,4),%rbx
  202441:	4b 8d 3c 1b                                     	lea    (%r11,%r11,1),%rdi
  202445:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
  202450:	4c 39 d8                                        	cmp    %r11,%rax
  202453:	0f 84 de 01 00 00                               	je     202637 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x467>
  202459:	4d 39 da                                        	cmp    %r11,%r10
  20245c:	0f 84 eb 01 00 00                               	je     20264d <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x47d>
  202462:	42 8b 4c 9b fc                                  	mov    -0x4(%rbx,%r11,4),%ecx
  202467:	46 8b 34 9b                                     	mov    (%rbx,%r11,4),%r14d
  20246b:	44 01 f1                                        	add    %r14d,%ecx
  20246e:	83 c1 02                                        	add    $0x2,%ecx
  202471:	c1 f9 02                                        	sar    $0x2,%ecx
  202474:	42 03 0c da                                     	add    (%rdx,%r11,8),%ecx
  202478:	43 89 0c 98                                     	mov    %ecx,(%r8,%r11,4)
  20247c:	49 ff c3                                        	inc    %r11
  20247f:	48 83 c7 02                                     	add    $0x2,%rdi
  202483:	4c 39 de                                        	cmp    %r11,%rsi
  202486:	75 c8                                           	jne    202450 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x280>
  202488:	e9 64 01 00 00                                  	jmp    2025f1 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x421>
  20248d:	48 89 f8                                        	mov    %rdi,%rax
  202490:	4c 8d 75 01                                     	lea    0x1(%rbp),%r14
  202494:	49 d1 ee                                        	shr    $1,%r14
  202497:	48 39 f7                                        	cmp    %rsi,%rdi
  20249a:	74 76                                           	je     202512 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x342>
  20249c:	49 89 ef                                        	mov    %rbp,%r15
  20249f:	49 d1 ef                                        	shr    $1,%r15
  2024a2:	4f 8d 24 36                                     	lea    (%r14,%r14,1),%r12
  2024a6:	31 ff                                           	xor    %edi,%edi
  2024a8:	49 89 f2                                        	mov    %rsi,%r10
  2024ab:	31 c9                                           	xor    %ecx,%ecx
  2024ad:	0f 1f 00                                        	nopl   (%rax)
  2024b0:	49 39 fc                                        	cmp    %rdi,%r12
  2024b3:	0f 84 a4 01 00 00                               	je     20265d <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x48d>
  2024b9:	4c 8d 69 01                                     	lea    0x1(%rcx),%r13
  2024bd:	49 39 f5                                        	cmp    %rsi,%r13
  2024c0:	49 89 cb                                        	mov    %rcx,%r11
  2024c3:	4d 0f 42 dd                                     	cmovb  %r13,%r11
  2024c7:	4d 01 db                                        	add    %r11,%r11
  2024ca:	49 39 eb                                        	cmp    %rbp,%r11
  2024cd:	0f 83 c0 01 00 00                               	jae    202693 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x4c3>
  2024d3:	4c 39 f9                                        	cmp    %r15,%rcx
  2024d6:	0f 84 91 01 00 00                               	je     20266d <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x49d>
  2024dc:	4d 39 ca                                        	cmp    %r9,%r10
  2024df:	0f 83 9b 01 00 00                               	jae    202680 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x4b0>
  2024e5:	8b 0c ba                                        	mov    (%rdx,%rdi,4),%ecx
  2024e8:	42 03 0c 9a                                     	add    (%rdx,%r11,4),%ecx
  2024ec:	44 8b 5c ba 04                                  	mov    0x4(%rdx,%rdi,4),%r11d
  2024f1:	d1 f9                                           	sar    $1,%ecx
  2024f3:	41 29 cb                                        	sub    %ecx,%r11d
  2024f6:	47 89 1c 90                                     	mov    %r11d,(%r8,%r10,4)
  2024fa:	49 ff c2                                        	inc    %r10
  2024fd:	48 83 c7 02                                     	add    $0x2,%rdi
  202501:	4c 89 e9                                        	mov    %r13,%rcx
  202504:	49 39 dd                                        	cmp    %rbx,%r13
  202507:	75 a7                                           	jne    2024b0 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x2e0>
  202509:	48 85 f6                                        	test   %rsi,%rsi
  20250c:	0f 84 df 00 00 00                               	je     2025f1 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x421>
  202512:	48 89 2c 24                                     	mov    %rbp,(%rsp)
  202516:	49 39 f1                                        	cmp    %rsi,%r9
  202519:	0f 86 34 02 00 00                               	jbe    202753 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x583>
  20251f:	48 8d 48 ff                                     	lea    -0x1(%rax),%rcx
  202523:	4c 39 c9                                        	cmp    %r9,%rcx
  202526:	40 0f 93 c7                                     	setae  %dil
  20252a:	48 39 f0                                        	cmp    %rsi,%rax
  20252d:	41 0f 94 c2                                     	sete   %r10b
  202531:	41 84 fa                                        	test   %dil,%r10b
  202534:	0f 85 a6 01 00 00                               	jne    2026e0 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x510>
  20253a:	48 39 f0                                        	cmp    %rsi,%rax
  20253d:	48 89 c8                                        	mov    %rcx,%rax
  202540:	48 0f 45 c6                                     	cmovne %rsi,%rax
  202544:	41 8b 3c b0                                     	mov    (%r8,%rsi,4),%edi
  202548:	41 8b 04 80                                     	mov    (%r8,%rax,4),%eax
  20254c:	01 f8                                           	add    %edi,%eax
  20254e:	83 c0 02                                        	add    $0x2,%eax
  202551:	c1 f8 02                                        	sar    $0x2,%eax
  202554:	03 02                                           	add    (%rdx),%eax
  202556:	41 89 00                                        	mov    %eax,(%r8)
  202559:	48 83 fe 01                                     	cmp    $0x1,%rsi
  20255d:	0f 84 8e 00 00 00                               	je     2025f1 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x421>
  202563:	4d 8d 59 ff                                     	lea    -0x1(%r9),%r11
  202567:	49 ff ce                                        	dec    %r14
  20256a:	49 89 f7                                        	mov    %rsi,%r15
  20256d:	4d 29 cf                                        	sub    %r9,%r15
  202570:	4c 8d 66 ff                                     	lea    -0x1(%rsi),%r12
  202574:	b8 02 00 00 00                                  	mov    $0x2,%eax
  202579:	45 31 ed                                        	xor    %r13d,%r13d
  20257c:	0f 1f 40 00                                     	nopl   0x0(%rax)
  202580:	4c 89 ff                                        	mov    %r15,%rdi
  202583:	4c 01 ef                                        	add    %r13,%rdi
  202586:	0f 84 1a 01 00 00                               	je     2026a6 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x4d6>
  20258c:	49 8d 6d 01                                     	lea    0x1(%r13),%rbp
  202590:	49 8d 3c b0                                     	lea    (%r8,%rsi,4),%rdi
  202594:	46 8b 14 af                                     	mov    (%rdi,%r13,4),%r10d
  202598:	48 39 dd                                        	cmp    %rbx,%rbp
  20259b:	73 13                                           	jae    2025b0 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x3e0>
  20259d:	4a 8d 3c 2e                                     	lea    (%rsi,%r13,1),%rdi
  2025a1:	48 ff c7                                        	inc    %rdi
  2025a4:	4c 39 cf                                        	cmp    %r9,%rdi
  2025a7:	72 13                                           	jb     2025bc <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x3ec>
  2025a9:	e9 45 01 00 00                                  	jmp    2026f3 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x523>
  2025ae:	66 90                                           	xchg   %ax,%ax
  2025b0:	48 89 cf                                        	mov    %rcx,%rdi
  2025b3:	4c 39 c9                                        	cmp    %r9,%rcx
  2025b6:	0f 83 24 01 00 00                               	jae    2026e0 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x510>
  2025bc:	4d 39 ee                                        	cmp    %r13,%r14
  2025bf:	0f 84 f4 00 00 00                               	je     2026b9 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x4e9>
  2025c5:	4d 39 eb                                        	cmp    %r13,%r11
  2025c8:	0f 84 ff 00 00 00                               	je     2026cd <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x4fd>
  2025ce:	41 8b 3c b8                                     	mov    (%r8,%rdi,4),%edi
  2025d2:	44 01 d7                                        	add    %r10d,%edi
  2025d5:	83 c7 02                                        	add    $0x2,%edi
  2025d8:	c1 ff 02                                        	sar    $0x2,%edi
  2025db:	42 03 7c ea 08                                  	add    0x8(%rdx,%r13,8),%edi
  2025e0:	43 89 7c a8 04                                  	mov    %edi,0x4(%r8,%r13,4)
  2025e5:	48 83 c0 02                                     	add    $0x2,%rax
  2025e9:	49 89 ed                                        	mov    %rbp,%r13
  2025ec:	49 39 ec                                        	cmp    %rbp,%r12
  2025ef:	75 8f                                           	jne    202580 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x3b0>
  2025f1:	48 83 c4 08                                     	add    $0x8,%rsp
  2025f5:	5b                                              	pop    %rbx
  2025f6:	41 5c                                           	pop    %r12
  2025f8:	41 5d                                           	pop    %r13
  2025fa:	41 5e                                           	pop    %r14
  2025fc:	41 5f                                           	pop    %r15
  2025fe:	5d                                              	pop    %rbp
  2025ff:	c3                                              	ret
  202600:	48 8d 15 49 8b 06 00                            	lea    0x68b49(%rip),%rdx        # 26b150 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x41a8>
  202607:	48 89 ee                                        	mov    %rbp,%rsi
  20260a:	ff 15 98 b7 06 00                               	call   *0x6b798(%rip)        # 26dda8 <_DYNAMIC+0x228>
  202610:	48 83 c7 02                                     	add    $0x2,%rdi
  202614:	48 8d 15 4d 8b 06 00                            	lea    0x68b4d(%rip),%rdx        # 26b168 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x41c0>
  20261b:	48 89 ee                                        	mov    %rbp,%rsi
  20261e:	ff 15 84 b7 06 00                               	call   *0x6b784(%rip)        # 26dda8 <_DYNAMIC+0x228>
  202624:	48 8d 15 55 8b 06 00                            	lea    0x68b55(%rip),%rdx        # 26b180 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x41d8>
  20262b:	48 89 c7                                        	mov    %rax,%rdi
  20262e:	4c 89 ce                                        	mov    %r9,%rsi
  202631:	ff 15 71 b7 06 00                               	call   *0x6b771(%rip)        # 26dda8 <_DYNAMIC+0x228>
  202637:	4c 01 de                                        	add    %r11,%rsi
  20263a:	48 8d 15 df 8a 06 00                            	lea    0x68adf(%rip),%rdx        # 26b120 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x4178>
  202641:	48 89 f7                                        	mov    %rsi,%rdi
  202644:	4c 89 ce                                        	mov    %r9,%rsi
  202647:	ff 15 5b b7 06 00                               	call   *0x6b75b(%rip)        # 26dda8 <_DYNAMIC+0x228>
  20264d:	48 8d 15 e4 8a 06 00                            	lea    0x68ae4(%rip),%rdx        # 26b138 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x4190>
  202654:	48 89 ee                                        	mov    %rbp,%rsi
  202657:	ff 15 4b b7 06 00                               	call   *0x6b74b(%rip)        # 26dda8 <_DYNAMIC+0x228>
  20265d:	48 8d 15 2c 87 06 00                            	lea    0x6872c(%rip),%rdx        # 26ad90 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3de8>
  202664:	48 89 ee                                        	mov    %rbp,%rsi
  202667:	ff 15 3b b7 06 00                               	call   *0x6b73b(%rip)        # 26dda8 <_DYNAMIC+0x228>
  20266d:	48 ff c7                                        	inc    %rdi
  202670:	48 8d 15 49 87 06 00                            	lea    0x68749(%rip),%rdx        # 26adc0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3e18>
  202677:	48 89 ee                                        	mov    %rbp,%rsi
  20267a:	ff 15 28 b7 06 00                               	call   *0x6b728(%rip)        # 26dda8 <_DYNAMIC+0x228>
  202680:	48 8d 15 51 87 06 00                            	lea    0x68751(%rip),%rdx        # 26add8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3e30>
  202687:	4c 89 d7                                        	mov    %r10,%rdi
  20268a:	4c 89 ce                                        	mov    %r9,%rsi
  20268d:	ff 15 15 b7 06 00                               	call   *0x6b715(%rip)        # 26dda8 <_DYNAMIC+0x228>
  202693:	48 8d 15 0e 87 06 00                            	lea    0x6870e(%rip),%rdx        # 26ada8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3e00>
  20269a:	4c 89 df                                        	mov    %r11,%rdi
  20269d:	48 89 ee                                        	mov    %rbp,%rsi
  2026a0:	ff 15 02 b7 06 00                               	call   *0x6b702(%rip)        # 26dda8 <_DYNAMIC+0x228>
  2026a6:	48 8d 15 6b 86 06 00                            	lea    0x6866b(%rip),%rdx        # 26ad18 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3d70>
  2026ad:	4c 89 cf                                        	mov    %r9,%rdi
  2026b0:	4c 89 ce                                        	mov    %r9,%rsi
  2026b3:	ff 15 ef b6 06 00                               	call   *0x6b6ef(%rip)        # 26dda8 <_DYNAMIC+0x228>
  2026b9:	48 8d 15 a0 86 06 00                            	lea    0x686a0(%rip),%rdx        # 26ad60 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3db8>
  2026c0:	48 89 c7                                        	mov    %rax,%rdi
  2026c3:	48 8b 34 24                                     	mov    (%rsp),%rsi
  2026c7:	ff 15 db b6 06 00                               	call   *0x6b6db(%rip)        # 26dda8 <_DYNAMIC+0x228>
  2026cd:	48 8d 15 a4 86 06 00                            	lea    0x686a4(%rip),%rdx        # 26ad78 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3dd0>
  2026d4:	4c 89 cf                                        	mov    %r9,%rdi
  2026d7:	4c 89 ce                                        	mov    %r9,%rsi
  2026da:	ff 15 c8 b6 06 00                               	call   *0x6b6c8(%rip)        # 26dda8 <_DYNAMIC+0x228>
  2026e0:	48 8d 15 49 86 06 00                            	lea    0x68649(%rip),%rdx        # 26ad30 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3d88>
  2026e7:	48 89 cf                                        	mov    %rcx,%rdi
  2026ea:	4c 89 ce                                        	mov    %r9,%rsi
  2026ed:	ff 15 b5 b6 06 00                               	call   *0x6b6b5(%rip)        # 26dda8 <_DYNAMIC+0x228>
  2026f3:	4a 8d 3c 2e                                     	lea    (%rsi,%r13,1),%rdi
  2026f7:	48 ff c7                                        	inc    %rdi
  2026fa:	48 8d 15 47 86 06 00                            	lea    0x68647(%rip),%rdx        # 26ad48 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3da0>
  202701:	4c 89 ce                                        	mov    %r9,%rsi
  202704:	ff 15 9e b6 06 00                               	call   *0x6b69e(%rip)        # 26dda8 <_DYNAMIC+0x228>
  20270a:	48 8d 15 af 89 06 00                            	lea    0x689af(%rip),%rdx        # 26b0c0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x4118>
  202711:	48 89 ee                                        	mov    %rbp,%rsi
  202714:	ff 15 8e b6 06 00                               	call   *0x6b68e(%rip)        # 26dda8 <_DYNAMIC+0x228>
  20271a:	48 8d 15 b7 89 06 00                            	lea    0x689b7(%rip),%rdx        # 26b0d8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x4130>
  202721:	4c 89 d7                                        	mov    %r10,%rdi
  202724:	48 89 ee                                        	mov    %rbp,%rsi
  202727:	ff 15 7b b6 06 00                               	call   *0x6b67b(%rip)        # 26dda8 <_DYNAMIC+0x228>
  20272d:	48 8d 15 bc 89 06 00                            	lea    0x689bc(%rip),%rdx        # 26b0f0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x4148>
  202734:	48 89 c7                                        	mov    %rax,%rdi
  202737:	4c 89 ce                                        	mov    %r9,%rsi
  20273a:	ff 15 68 b6 06 00                               	call   *0x6b668(%rip)        # 26dda8 <_DYNAMIC+0x228>
  202740:	48 8d 15 c1 89 06 00                            	lea    0x689c1(%rip),%rdx        # 26b108 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x4160>
  202747:	48 89 f7                                        	mov    %rsi,%rdi
  20274a:	4c 89 ce                                        	mov    %r9,%rsi
  20274d:	ff 15 55 b6 06 00                               	call   *0x6b655(%rip)        # 26dda8 <_DYNAMIC+0x228>
  202753:	48 8d 15 a6 85 06 00                            	lea    0x685a6(%rip),%rdx        # 26ad00 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3d58>
  20275a:	48 89 f7                                        	mov    %rsi,%rdi
  20275d:	4c 89 ce                                        	mov    %r9,%rsi
  202760:	ff 15 42 b6 06 00                               	call   *0x6b642(%rip)        # 26dda8 <_DYNAMIC+0x228>
