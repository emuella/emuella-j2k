Disassembly of section .text:

0000000000206600 <emuella_j2k_transform::transform_line_forward_first_low_bounded>:
  206600:	55                                              	push   %rbp
  206601:	41 57                                           	push   %r15
  206603:	41 56                                           	push   %r14
  206605:	41 55                                           	push   %r13
  206607:	41 54                                           	push   %r12
  206609:	53                                              	push   %rbx
  20660a:	50                                              	push   %rax
  20660b:	48 89 fb                                        	mov    %rdi,%rbx
  20660e:	48 29 f3                                        	sub    %rsi,%rbx
  206611:	48 39 de                                        	cmp    %rbx,%rsi
  206614:	0f 85 a3 02 00 00                               	jne    2068bd <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x2bd>
  20661a:	48 83 fe 01                                     	cmp    $0x1,%rsi
  20661e:	49 89 f2                                        	mov    %rsi,%r10
  206621:	49 83 d2 ff                                     	adc    $0xffffffffffffffff,%r10
  206625:	48 83 fe 02                                     	cmp    $0x2,%rsi
  206629:	0f 82 02 01 00 00                               	jb     206731 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x131>
  20662f:	4c 8d 59 01                                     	lea    0x1(%rcx),%r11
  206633:	49 d1 eb                                        	shr    $1,%r11
  206636:	48 83 f9 03                                     	cmp    $0x3,%rcx
  20663a:	bb 02 00 00 00                                  	mov    $0x2,%ebx
  20663f:	48 0f 43 d9                                     	cmovae %rcx,%rbx
  206643:	48 ff cb                                        	dec    %rbx
  206646:	48 d1 eb                                        	shr    $1,%rbx
  206649:	4c 39 db                                        	cmp    %r11,%rbx
  20664c:	4c 89 df                                        	mov    %r11,%rdi
  20664f:	48 0f 42 fb                                     	cmovb  %rbx,%rdi
  206653:	48 8d 46 fe                                     	lea    -0x2(%rsi),%rax
  206657:	48 39 c7                                        	cmp    %rax,%rdi
  20665a:	48 0f 43 f8                                     	cmovae %rax,%rdi
  20665e:	45 31 f6                                        	xor    %r14d,%r14d
  206661:	4c 89 c8                                        	mov    %r9,%rax
  206664:	48 29 f0                                        	sub    %rsi,%rax
  206667:	49 0f 42 c6                                     	cmovb  %r14,%rax
  20666b:	48 39 c7                                        	cmp    %rax,%rdi
  20666e:	48 0f 42 c7                                     	cmovb  %rdi,%rax
  206672:	48 83 f8 03                                     	cmp    $0x3,%rax
  206676:	76 65                                           	jbe    2066dd <emuella_j2k_transform::transform_line_forward_first_low_bounded+0xdd>
  206678:	48 ff c0                                        	inc    %rax
  20667b:	41 89 c6                                        	mov    %eax,%r14d
  20667e:	41 83 e6 03                                     	and    $0x3,%r14d
  206682:	bf 04 00 00 00                                  	mov    $0x4,%edi
  206687:	49 0f 45 fe                                     	cmovne %r14,%rdi
  20668b:	49 89 c6                                        	mov    %rax,%r14
  20668e:	49 29 fe                                        	sub    %rdi,%r14
  206691:	49 8d 04 b0                                     	lea    (%r8,%rsi,4),%rax
  206695:	31 ff                                           	xor    %edi,%edi
  206697:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  2066a0:	0f 10 04 fa                                     	movups (%rdx,%rdi,8),%xmm0
  2066a4:	0f 10 4c fa 04                                  	movups 0x4(%rdx,%rdi,8),%xmm1
  2066a9:	0f 10 54 fa 10                                  	movups 0x10(%rdx,%rdi,8),%xmm2
  2066ae:	0f c6 c2 88                                     	shufps $0x88,%xmm2,%xmm0
  2066b2:	0f 10 54 fa 14                                  	movups 0x14(%rdx,%rdi,8),%xmm2
  2066b7:	0f 28 d9                                        	movaps %xmm1,%xmm3
  2066ba:	0f c6 da 88                                     	shufps $0x88,%xmm2,%xmm3
  2066be:	0f c6 ca dd                                     	shufps $0xdd,%xmm2,%xmm1
  2066c2:	66 0f fe c8                                     	paddd  %xmm0,%xmm1
  2066c6:	66 0f 72 e1 01                                  	psrad  $0x1,%xmm1
  2066cb:	66 0f fa d9                                     	psubd  %xmm1,%xmm3
  2066cf:	f3 0f 7f 1c b8                                  	movdqu %xmm3,(%rax,%rdi,4)
  2066d4:	48 83 c7 04                                     	add    $0x4,%rdi
  2066d8:	49 39 fe                                        	cmp    %rdi,%r14
  2066db:	75 c3                                           	jne    2066a0 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0xa0>
  2066dd:	4d 8d 3c b0                                     	lea    (%r8,%rsi,4),%r15
  2066e1:	4b 8d 3c 36                                     	lea    (%r14,%r14,1),%rdi
  2066e5:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
  2066f0:	4d 39 f3                                        	cmp    %r14,%r11
  2066f3:	0f 84 37 03 00 00                               	je     206a30 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x430>
  2066f9:	4c 39 f3                                        	cmp    %r14,%rbx
  2066fc:	0f 84 3e 03 00 00                               	je     206a40 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x440>
  206702:	4a 8d 04 36                                     	lea    (%rsi,%r14,1),%rax
  206706:	4c 39 c8                                        	cmp    %r9,%rax
  206709:	0f 83 45 03 00 00                               	jae    206a54 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x454>
  20670f:	42 8b 04 f2                                     	mov    (%rdx,%r14,8),%eax
  206713:	42 03 44 f2 08                                  	add    0x8(%rdx,%r14,8),%eax
  206718:	42 8b 6c f2 04                                  	mov    0x4(%rdx,%r14,8),%ebp
  20671d:	d1 f8                                           	sar    $1,%eax
  20671f:	29 c5                                           	sub    %eax,%ebp
  206721:	43 89 2c b7                                     	mov    %ebp,(%r15,%r14,4)
  206725:	49 ff c6                                        	inc    %r14
  206728:	48 83 c7 02                                     	add    $0x2,%rdi
  20672c:	4d 39 d6                                        	cmp    %r10,%r14
  20672f:	72 bf                                           	jb     2066f0 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0xf0>
  206731:	48 8d 3c 75 fe ff ff ff                         	lea    -0x2(,%rsi,2),%rdi
  206739:	48 39 cf                                        	cmp    %rcx,%rdi
  20673c:	0f 83 ea 03 00 00                               	jae    206b2c <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x52c>
  206742:	4c 8d 14 75 ff ff ff ff                         	lea    -0x1(,%rsi,2),%r10
  20674a:	49 39 ca                                        	cmp    %rcx,%r10
  20674d:	0f 83 e9 03 00 00                               	jae    206b3c <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x53c>
  206753:	48 8d 46 ff                                     	lea    -0x1(%rsi),%rax
  206757:	48 01 f0                                        	add    %rsi,%rax
  20675a:	4c 39 c8                                        	cmp    %r9,%rax
  20675d:	0f 83 ec 03 00 00                               	jae    206b4f <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x54f>
  206763:	49 89 cf                                        	mov    %rcx,%r15
  206766:	42 8b 0c 92                                     	mov    (%rdx,%r10,4),%ecx
  20676a:	8b 3c ba                                        	mov    (%rdx,%rdi,4),%edi
  20676d:	01 ff                                           	add    %edi,%edi
  20676f:	d1 ff                                           	sar    $1,%edi
  206771:	29 f9                                           	sub    %edi,%ecx
  206773:	41 89 0c 80                                     	mov    %ecx,(%r8,%rax,4)
  206777:	4c 89 c8                                        	mov    %r9,%rax
  20677a:	48 29 f0                                        	sub    %rsi,%rax
  20677d:	0f 86 df 03 00 00                               	jbe    206b62 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x562>
  206783:	41 8b 0c b0                                     	mov    (%r8,%rsi,4),%ecx
  206787:	8d 0c 4d 02 00 00 00                            	lea    0x2(,%rcx,2),%ecx
  20678e:	c1 f9 02                                        	sar    $0x2,%ecx
  206791:	03 0a                                           	add    (%rdx),%ecx
  206793:	41 89 08                                        	mov    %ecx,(%r8)
  206796:	48 83 fe 02                                     	cmp    $0x2,%rsi
  20679a:	0f 82 81 02 00 00                               	jb     206a21 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x421>
  2067a0:	4d 8d 57 ff                                     	lea    -0x1(%r15),%r10
  2067a4:	49 d1 ea                                        	shr    $1,%r10
  2067a7:	48 89 f7                                        	mov    %rsi,%rdi
  2067aa:	48 f7 d7                                        	not    %rdi
  2067ad:	4c 01 cf                                        	add    %r9,%rdi
  2067b0:	49 39 fa                                        	cmp    %rdi,%r10
  2067b3:	49 0f 42 fa                                     	cmovb  %r10,%rdi
  2067b7:	48 8d 4e fe                                     	lea    -0x2(%rsi),%rcx
  2067bb:	48 39 cf                                        	cmp    %rcx,%rdi
  2067be:	48 0f 43 f9                                     	cmovae %rcx,%rdi
  2067c2:	41 bb 01 00 00 00                               	mov    $0x1,%r11d
  2067c8:	48 83 ff 08                                     	cmp    $0x8,%rdi
  2067cc:	0f 82 98 00 00 00                               	jb     20686a <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x26a>
  2067d2:	48 8d 0c b5 0b 00 00 00                         	lea    0xb(,%rsi,4),%rcx
  2067da:	48 83 f9 10                                     	cmp    $0x10,%rcx
  2067de:	0f 82 86 00 00 00                               	jb     20686a <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x26a>
  2067e4:	48 8d 0c b5 00 00 00 00                         	lea    0x0(,%rsi,4),%rcx
  2067ec:	48 89 cb                                        	mov    %rcx,%rbx
  2067ef:	48 f7 db                                        	neg    %rbx
  2067f2:	48 83 fb 10                                     	cmp    $0x10,%rbx
  2067f6:	72 72                                           	jb     20686a <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x26a>
  2067f8:	48 ff c7                                        	inc    %rdi
  2067fb:	41 89 fb                                        	mov    %edi,%r11d
  2067fe:	41 83 e3 03                                     	and    $0x3,%r11d
  206802:	41 be 04 00 00 00                               	mov    $0x4,%r14d
  206808:	4d 0f 45 f3                                     	cmovne %r11,%r14
  20680c:	48 89 fb                                        	mov    %rdi,%rbx
  20680f:	4c 29 f3                                        	sub    %r14,%rbx
  206812:	49 f7 de                                        	neg    %r14
  206815:	4e 8d 1c 37                                     	lea    (%rdi,%r14,1),%r11
  206819:	49 ff c3                                        	inc    %r11
  20681c:	4c 01 c1                                        	add    %r8,%rcx
  20681f:	48 83 c1 04                                     	add    $0x4,%rcx
  206823:	31 ff                                           	xor    %edi,%edi
  206825:	66 0f 6f 05 83 9f e0 ff                         	movdqa -0x1f607d(%rip),%xmm0        # 107b0 <anon.ee651107ab5319c6bc273e1a29320aaf.78.llvm.14746981713632465754+0x40>
  20682d:	0f 1f 00                                        	nopl   (%rax)
  206830:	0f 10 4c fa 08                                  	movups 0x8(%rdx,%rdi,8),%xmm1
  206835:	0f 10 54 fa 18                                  	movups 0x18(%rdx,%rdi,8),%xmm2
  20683a:	0f c6 ca 88                                     	shufps $0x88,%xmm2,%xmm1
  20683e:	f3 0f 6f 54 b9 fc                               	movdqu -0x4(%rcx,%rdi,4),%xmm2
  206844:	f3 0f 6f 1c b9                                  	movdqu (%rcx,%rdi,4),%xmm3
  206849:	66 0f fe da                                     	paddd  %xmm2,%xmm3
  20684d:	66 0f fe d8                                     	paddd  %xmm0,%xmm3
  206851:	66 0f 72 e3 02                                  	psrad  $0x2,%xmm3
  206856:	66 0f fe d9                                     	paddd  %xmm1,%xmm3
  20685a:	f3 41 0f 7f 5c b8 04                            	movdqu %xmm3,0x4(%r8,%rdi,4)
  206861:	48 83 c7 04                                     	add    $0x4,%rdi
  206865:	48 39 fb                                        	cmp    %rdi,%rbx
  206868:	75 c6                                           	jne    206830 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x230>
  20686a:	49 ff c2                                        	inc    %r10
  20686d:	49 8d 1c b0                                     	lea    (%r8,%rsi,4),%rbx
  206871:	4b 8d 3c 1b                                     	lea    (%r11,%r11,1),%rdi
  206875:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
  206880:	4c 39 d8                                        	cmp    %r11,%rax
  206883:	0f 84 de 01 00 00                               	je     206a67 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x467>
  206889:	4d 39 da                                        	cmp    %r11,%r10
  20688c:	0f 84 eb 01 00 00                               	je     206a7d <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x47d>
  206892:	42 8b 4c 9b fc                                  	mov    -0x4(%rbx,%r11,4),%ecx
  206897:	46 8b 34 9b                                     	mov    (%rbx,%r11,4),%r14d
  20689b:	44 01 f1                                        	add    %r14d,%ecx
  20689e:	83 c1 02                                        	add    $0x2,%ecx
  2068a1:	c1 f9 02                                        	sar    $0x2,%ecx
  2068a4:	42 03 0c da                                     	add    (%rdx,%r11,8),%ecx
  2068a8:	43 89 0c 98                                     	mov    %ecx,(%r8,%r11,4)
  2068ac:	49 ff c3                                        	inc    %r11
  2068af:	48 83 c7 02                                     	add    $0x2,%rdi
  2068b3:	4c 39 de                                        	cmp    %r11,%rsi
  2068b6:	75 c8                                           	jne    206880 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x280>
  2068b8:	e9 64 01 00 00                                  	jmp    206a21 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x421>
  2068bd:	48 39 f7                                        	cmp    %rsi,%rdi
  2068c0:	74 77                                           	je     206939 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x339>
  2068c2:	4c 8d 71 01                                     	lea    0x1(%rcx),%r14
  2068c6:	49 89 cf                                        	mov    %rcx,%r15
  2068c9:	49 d1 ef                                        	shr    $1,%r15
  2068cc:	49 83 e6 fe                                     	and    $0xfffffffffffffffe,%r14
  2068d0:	31 c0                                           	xor    %eax,%eax
  2068d2:	49 89 f2                                        	mov    %rsi,%r10
  2068d5:	45 31 ed                                        	xor    %r13d,%r13d
  2068d8:	0f 1f 84 00 00 00 00 00                         	nopl   0x0(%rax,%rax,1)
  2068e0:	49 39 c6                                        	cmp    %rax,%r14
  2068e3:	0f 84 a4 01 00 00                               	je     206a8d <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x48d>
  2068e9:	4d 8d 65 01                                     	lea    0x1(%r13),%r12
  2068ed:	49 39 f4                                        	cmp    %rsi,%r12
  2068f0:	4d 89 eb                                        	mov    %r13,%r11
  2068f3:	4d 0f 42 dc                                     	cmovb  %r12,%r11
  2068f7:	4d 01 db                                        	add    %r11,%r11
  2068fa:	49 39 cb                                        	cmp    %rcx,%r11
  2068fd:	0f 83 c6 01 00 00                               	jae    206ac9 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x4c9>
  206903:	4d 39 fd                                        	cmp    %r15,%r13
  206906:	0f 84 94 01 00 00                               	je     206aa0 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x4a0>
  20690c:	4d 39 ca                                        	cmp    %r9,%r10
  20690f:	0f 83 a1 01 00 00                               	jae    206ab6 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x4b6>
  206915:	8b 2c 82                                        	mov    (%rdx,%rax,4),%ebp
  206918:	42 03 2c 9a                                     	add    (%rdx,%r11,4),%ebp
  20691c:	44 8b 5c 82 04                                  	mov    0x4(%rdx,%rax,4),%r11d
  206921:	d1 fd                                           	sar    $1,%ebp
  206923:	41 29 eb                                        	sub    %ebp,%r11d
  206926:	47 89 1c 90                                     	mov    %r11d,(%r8,%r10,4)
  20692a:	49 ff c2                                        	inc    %r10
  20692d:	48 83 c0 02                                     	add    $0x2,%rax
  206931:	4d 89 e5                                        	mov    %r12,%r13
  206934:	49 39 dc                                        	cmp    %rbx,%r12
  206937:	75 a7                                           	jne    2068e0 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x2e0>
  206939:	48 89 0c 24                                     	mov    %rcx,(%rsp)
  20693d:	48 85 f6                                        	test   %rsi,%rsi
  206940:	0f 84 db 00 00 00                               	je     206a21 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x421>
  206946:	49 39 f1                                        	cmp    %rsi,%r9
  206949:	0f 86 26 02 00 00                               	jbe    206b75 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x575>
  20694f:	48 8b 04 24                                     	mov    (%rsp),%rax
  206953:	4c 8d 50 01                                     	lea    0x1(%rax),%r10
  206957:	49 d1 ea                                        	shr    $1,%r10
  20695a:	0f 84 28 02 00 00                               	je     206b88 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x588>
  206960:	48 8d 4f ff                                     	lea    -0x1(%rdi),%rcx
  206964:	48 39 f7                                        	cmp    %rsi,%rdi
  206967:	48 89 f0                                        	mov    %rsi,%rax
  20696a:	48 0f 44 c1                                     	cmove  %rcx,%rax
  20696e:	41 8b 3c b0                                     	mov    (%r8,%rsi,4),%edi
  206972:	41 8b 04 80                                     	mov    (%r8,%rax,4),%eax
  206976:	01 f8                                           	add    %edi,%eax
  206978:	83 c0 02                                        	add    $0x2,%eax
  20697b:	c1 f8 02                                        	sar    $0x2,%eax
  20697e:	03 02                                           	add    (%rdx),%eax
  206980:	41 89 00                                        	mov    %eax,(%r8)
  206983:	48 83 fe 01                                     	cmp    $0x1,%rsi
  206987:	0f 84 94 00 00 00                               	je     206a21 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x421>
  20698d:	4d 8d 71 ff                                     	lea    -0x1(%r9),%r14
  206991:	49 ff ca                                        	dec    %r10
  206994:	49 89 f7                                        	mov    %rsi,%r15
  206997:	4d 29 cf                                        	sub    %r9,%r15
  20699a:	4c 8d 66 ff                                     	lea    -0x1(%rsi),%r12
  20699e:	b8 02 00 00 00                                  	mov    $0x2,%eax
  2069a3:	45 31 ed                                        	xor    %r13d,%r13d
  2069a6:	66 2e 0f 1f 84 00 00 00 00 00                   	cs nopw 0x0(%rax,%rax,1)
  2069b0:	4c 89 ff                                        	mov    %r15,%rdi
  2069b3:	4c 01 ef                                        	add    %r13,%rdi
  2069b6:	0f 84 20 01 00 00                               	je     206adc <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x4dc>
  2069bc:	49 8d 6d 01                                     	lea    0x1(%r13),%rbp
  2069c0:	49 8d 3c b0                                     	lea    (%r8,%rsi,4),%rdi
  2069c4:	46 8b 1c af                                     	mov    (%rdi,%r13,4),%r11d
  2069c8:	48 39 dd                                        	cmp    %rbx,%rbp
  2069cb:	73 13                                           	jae    2069e0 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x3e0>
  2069cd:	4a 8d 3c 2e                                     	lea    (%rsi,%r13,1),%rdi
  2069d1:	48 ff c7                                        	inc    %rdi
  2069d4:	4c 39 cf                                        	cmp    %r9,%rdi
  2069d7:	72 13                                           	jb     2069ec <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x3ec>
  2069d9:	e9 24 01 00 00                                  	jmp    206b02 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x502>
  2069de:	66 90                                           	xchg   %ax,%ax
  2069e0:	48 89 cf                                        	mov    %rcx,%rdi
  2069e3:	4c 39 c9                                        	cmp    %r9,%rcx
  2069e6:	0f 83 2d 01 00 00                               	jae    206b19 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x519>
  2069ec:	4d 39 ea                                        	cmp    %r13,%r10
  2069ef:	0f 84 95 01 00 00                               	je     206b8a <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x58a>
  2069f5:	4d 39 ee                                        	cmp    %r13,%r14
  2069f8:	0f 84 f1 00 00 00                               	je     206aef <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x4ef>
  2069fe:	41 8b 3c b8                                     	mov    (%r8,%rdi,4),%edi
  206a02:	44 01 df                                        	add    %r11d,%edi
  206a05:	83 c7 02                                        	add    $0x2,%edi
  206a08:	c1 ff 02                                        	sar    $0x2,%edi
  206a0b:	42 03 7c ea 08                                  	add    0x8(%rdx,%r13,8),%edi
  206a10:	43 89 7c a8 04                                  	mov    %edi,0x4(%r8,%r13,4)
  206a15:	48 83 c0 02                                     	add    $0x2,%rax
  206a19:	49 89 ed                                        	mov    %rbp,%r13
  206a1c:	49 39 ec                                        	cmp    %rbp,%r12
  206a1f:	75 8f                                           	jne    2069b0 <emuella_j2k_transform::transform_line_forward_first_low_bounded+0x3b0>
  206a21:	48 83 c4 08                                     	add    $0x8,%rsp
  206a25:	5b                                              	pop    %rbx
  206a26:	41 5c                                           	pop    %r12
  206a28:	41 5d                                           	pop    %r13
  206a2a:	41 5e                                           	pop    %r14
  206a2c:	41 5f                                           	pop    %r15
  206a2e:	5d                                              	pop    %rbp
  206a2f:	c3                                              	ret
  206a30:	48 8d 15 99 b5 06 00                            	lea    0x6b599(%rip),%rdx        # 271fd0 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x4358>
  206a37:	48 89 ce                                        	mov    %rcx,%rsi
  206a3a:	ff 15 d0 e3 06 00                               	call   *0x6e3d0(%rip)        # 274e10 <_DYNAMIC+0x260>
  206a40:	48 83 c7 02                                     	add    $0x2,%rdi
  206a44:	48 8d 15 9d b5 06 00                            	lea    0x6b59d(%rip),%rdx        # 271fe8 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x4370>
  206a4b:	48 89 ce                                        	mov    %rcx,%rsi
  206a4e:	ff 15 bc e3 06 00                               	call   *0x6e3bc(%rip)        # 274e10 <_DYNAMIC+0x260>
  206a54:	48 8d 15 a5 b5 06 00                            	lea    0x6b5a5(%rip),%rdx        # 272000 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x4388>
  206a5b:	48 89 c7                                        	mov    %rax,%rdi
  206a5e:	4c 89 ce                                        	mov    %r9,%rsi
  206a61:	ff 15 a9 e3 06 00                               	call   *0x6e3a9(%rip)        # 274e10 <_DYNAMIC+0x260>
  206a67:	4c 01 de                                        	add    %r11,%rsi
  206a6a:	48 8d 15 2f b5 06 00                            	lea    0x6b52f(%rip),%rdx        # 271fa0 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x4328>
  206a71:	48 89 f7                                        	mov    %rsi,%rdi
  206a74:	4c 89 ce                                        	mov    %r9,%rsi
  206a77:	ff 15 93 e3 06 00                               	call   *0x6e393(%rip)        # 274e10 <_DYNAMIC+0x260>
  206a7d:	48 8d 15 34 b5 06 00                            	lea    0x6b534(%rip),%rdx        # 271fb8 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x4340>
  206a84:	4c 89 fe                                        	mov    %r15,%rsi
  206a87:	ff 15 83 e3 06 00                               	call   *0x6e383(%rip)        # 274e10 <_DYNAMIC+0x260>
  206a8d:	48 8d 15 7c b1 06 00                            	lea    0x6b17c(%rip),%rdx        # 271c10 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3f98>
  206a94:	48 89 c7                                        	mov    %rax,%rdi
  206a97:	48 89 ce                                        	mov    %rcx,%rsi
  206a9a:	ff 15 70 e3 06 00                               	call   *0x6e370(%rip)        # 274e10 <_DYNAMIC+0x260>
  206aa0:	48 ff c0                                        	inc    %rax
  206aa3:	48 8d 15 96 b1 06 00                            	lea    0x6b196(%rip),%rdx        # 271c40 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3fc8>
  206aaa:	48 89 c7                                        	mov    %rax,%rdi
  206aad:	48 89 ce                                        	mov    %rcx,%rsi
  206ab0:	ff 15 5a e3 06 00                               	call   *0x6e35a(%rip)        # 274e10 <_DYNAMIC+0x260>
  206ab6:	48 8d 15 9b b1 06 00                            	lea    0x6b19b(%rip),%rdx        # 271c58 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3fe0>
  206abd:	4c 89 d7                                        	mov    %r10,%rdi
  206ac0:	4c 89 ce                                        	mov    %r9,%rsi
  206ac3:	ff 15 47 e3 06 00                               	call   *0x6e347(%rip)        # 274e10 <_DYNAMIC+0x260>
  206ac9:	48 8d 15 58 b1 06 00                            	lea    0x6b158(%rip),%rdx        # 271c28 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3fb0>
  206ad0:	4c 89 df                                        	mov    %r11,%rdi
  206ad3:	48 89 ce                                        	mov    %rcx,%rsi
  206ad6:	ff 15 34 e3 06 00                               	call   *0x6e334(%rip)        # 274e10 <_DYNAMIC+0x260>
  206adc:	48 8d 15 b5 b0 06 00                            	lea    0x6b0b5(%rip),%rdx        # 271b98 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3f20>
  206ae3:	4c 89 cf                                        	mov    %r9,%rdi
  206ae6:	4c 89 ce                                        	mov    %r9,%rsi
  206ae9:	ff 15 21 e3 06 00                               	call   *0x6e321(%rip)        # 274e10 <_DYNAMIC+0x260>
  206aef:	48 8d 15 02 b1 06 00                            	lea    0x6b102(%rip),%rdx        # 271bf8 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3f80>
  206af6:	4c 89 cf                                        	mov    %r9,%rdi
  206af9:	4c 89 ce                                        	mov    %r9,%rsi
  206afc:	ff 15 0e e3 06 00                               	call   *0x6e30e(%rip)        # 274e10 <_DYNAMIC+0x260>
  206b02:	4a 8d 3c 2e                                     	lea    (%rsi,%r13,1),%rdi
  206b06:	48 ff c7                                        	inc    %rdi
  206b09:	48 8d 15 b8 b0 06 00                            	lea    0x6b0b8(%rip),%rdx        # 271bc8 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3f50>
  206b10:	4c 89 ce                                        	mov    %r9,%rsi
  206b13:	ff 15 f7 e2 06 00                               	call   *0x6e2f7(%rip)        # 274e10 <_DYNAMIC+0x260>
  206b19:	48 8d 15 90 b0 06 00                            	lea    0x6b090(%rip),%rdx        # 271bb0 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3f38>
  206b20:	48 89 cf                                        	mov    %rcx,%rdi
  206b23:	4c 89 ce                                        	mov    %r9,%rsi
  206b26:	ff 15 e4 e2 06 00                               	call   *0x6e2e4(%rip)        # 274e10 <_DYNAMIC+0x260>
  206b2c:	48 8d 15 0d b4 06 00                            	lea    0x6b40d(%rip),%rdx        # 271f40 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x42c8>
  206b33:	48 89 ce                                        	mov    %rcx,%rsi
  206b36:	ff 15 d4 e2 06 00                               	call   *0x6e2d4(%rip)        # 274e10 <_DYNAMIC+0x260>
  206b3c:	48 8d 15 15 b4 06 00                            	lea    0x6b415(%rip),%rdx        # 271f58 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x42e0>
  206b43:	4c 89 d7                                        	mov    %r10,%rdi
  206b46:	48 89 ce                                        	mov    %rcx,%rsi
  206b49:	ff 15 c1 e2 06 00                               	call   *0x6e2c1(%rip)        # 274e10 <_DYNAMIC+0x260>
  206b4f:	48 8d 15 1a b4 06 00                            	lea    0x6b41a(%rip),%rdx        # 271f70 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x42f8>
  206b56:	48 89 c7                                        	mov    %rax,%rdi
  206b59:	4c 89 ce                                        	mov    %r9,%rsi
  206b5c:	ff 15 ae e2 06 00                               	call   *0x6e2ae(%rip)        # 274e10 <_DYNAMIC+0x260>
  206b62:	48 8d 15 1f b4 06 00                            	lea    0x6b41f(%rip),%rdx        # 271f88 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x4310>
  206b69:	48 89 f7                                        	mov    %rsi,%rdi
  206b6c:	4c 89 ce                                        	mov    %r9,%rsi
  206b6f:	ff 15 9b e2 06 00                               	call   *0x6e29b(%rip)        # 274e10 <_DYNAMIC+0x260>
  206b75:	48 8d 15 04 b0 06 00                            	lea    0x6b004(%rip),%rdx        # 271b80 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3f08>
  206b7c:	48 89 f7                                        	mov    %rsi,%rdi
  206b7f:	4c 89 ce                                        	mov    %r9,%rsi
  206b82:	ff 15 88 e2 06 00                               	call   *0x6e288(%rip)        # 274e10 <_DYNAMIC+0x260>
  206b88:	31 c0                                           	xor    %eax,%eax
  206b8a:	48 8d 15 4f b0 06 00                            	lea    0x6b04f(%rip),%rdx        # 271be0 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3f68>
  206b91:	48 89 c7                                        	mov    %rax,%rdi
  206b94:	48 8b 34 24                                     	mov    (%rsp),%rsi
  206b98:	ff 15 72 e2 06 00                               	call   *0x6e272(%rip)        # 274e10 <_DYNAMIC+0x260>
