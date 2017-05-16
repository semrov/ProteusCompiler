<?xml version="1.0" encoding="ISO-8859-1"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

<xsl:template match="synanal">
	<html>
		<body>
			<xsl:apply-templates select="production"/>
		</body>
	</html>
</xsl:template>

<xsl:template match="production">
	<table cellspacing="0">
		<tr>
			<td align="center" style="background-color:#E69D26">
				<xsl:apply-templates select="leftside"/>
			</td>
		</tr>
		<tr>
			<td>
				<xsl:apply-templates select="rightside"/>
			</td>
		</tr>
	</table>
</xsl:template>

<xsl:template match="leftside">
	<xsl:variable name="nonterminal" select="@nonterminal"/>
	<text>
   		<xsl:value-of select="$nonterminal"/>
    </text>
</xsl:template>

<xsl:template match="rightside">
	<table cellspacing="0">
		<tr>
			<xsl:for-each select="production|symbol">
				<td valign="top">
					<xsl:apply-templates select="."/>
				</td>
			</xsl:for-each>
		</tr>
	</table>
</xsl:template>

<xsl:template match="symbol">
	<table cellspacing="0">
		<tr><td>
			<xsl:variable name="token" select="@token"/>
  			<xsl:variable name="lexeme" select="@lexeme"/>
    		<text style="background-color:#F5D02B">
    			<nobr><xsl:value-of select="$token"/><xsl:if test="$lexeme">=<xsl:value-of select="$lexeme"/></xsl:if><xsl:apply-templates select="position"/></nobr>
    		</text>
    	</td></tr>
    </table>
</xsl:template>

<xsl:template match="position">
	<xsl:variable name="begLine" select="@begLine"/>
	<xsl:variable name="begColumn" select="@begColumn"/>
	<xsl:variable name="endLine" select="@endLine"/>
	<xsl:variable name="endColumn" select="@endColumn"/>
	(<xsl:value-of select="$begLine"/>.<xsl:value-of select="$begColumn"/>:<xsl:value-of select="$endLine"/>.<xsl:value-of select="$endColumn"/>)
</xsl:template>

</xsl:stylesheet>