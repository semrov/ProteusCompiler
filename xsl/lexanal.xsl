<?xml version="1.0" encoding="ISO-8859-1"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

<xsl:template match="lexanal">
	<html>
		<body>
			<table cellspacing="0">
				<xsl:for-each select="symbol">
					<tr><td>
					<xsl:apply-templates select="."/>
					</td></tr>
				</xsl:for-each>
			</table>
		</body>
	</html>
</xsl:template>

<xsl:template match="symbol">
	<xsl:variable name="token" select="@token"/>
  	<xsl:variable name="lexeme" select="@lexeme"/>
    <text style="background-color:#F5D02B">
  		<xsl:apply-templates select="position"/>
    	<xsl:value-of select="$token"/><xsl:if test="$lexeme">=<xsl:value-of select="$lexeme"/></xsl:if>
    </text>
</xsl:template>

<xsl:template match="position">
	<xsl:variable name="begLine" select="@begLine"/>
	<xsl:variable name="begColumn" select="@begColumn"/>
	<xsl:variable name="endLine" select="@endLine"/>
	<xsl:variable name="endColumn" select="@endColumn"/>
	(<xsl:value-of select="$begLine"/>.<xsl:value-of select="$begColumn"/>:<xsl:value-of select="$endLine"/>.<xsl:value-of select="$endColumn"/>)
</xsl:template>

</xsl:stylesheet>